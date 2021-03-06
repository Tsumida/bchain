//! # 拜占庭将军问题
//! 拜占庭将军问题是指若干个将军要达成共识，也就是要同时满足如下条件:
//! 1. 所有忠诚的将军都作出相同的决定。
//! 2. 如果指挥官是忠诚的，那么所有忠诚的将军都执行他的命令。
//! 
//! 上述两个条件称为交互式一致性(interative consistency). 将军中可能有叛徒(traitor)，叛徒可能会迷惑、干扰忠诚的将军们作出决定。
//! 
//! Lamport 在其论文中给出的 oral message算法， 其假设是：
//! 1. 系统节点数量为N， 最多有f个拜占庭节点，且满足 N > 3f
//! 2. 存在可信信道， 也就是msg不会被篡改、丢失、延迟 
//! 3. 将军可以知道msg是谁发来的
//! 4. 将军可以知道是否缺少某条msg
//! 5. 任意两个将军都可以通信
//! 
//! # OM(m)
//! 采用了主从的角色: 司令官(Commander)将消息发给他的副官(Lieutenant).
//! OM(m)采用了递归形式，其步骤如下:
//! 
//! 当 m = 0:
//! 1. 司令官给每个副官发送他的值
//! 1. 如果副官收到了这个值，采用它；如果没收到，选择撤退。
//! 
//! 当 m > 0:
//! 1. 司令官给每个副官Li发送值 vi
//! 1. 对于每个副官, id=i, 其收到的值为vi, 如果没收到，采取默认值撤退。每个副官Li作为司令官向其他n-2个L执行om(m-1)
//! 1. 对于每个 j != i, vj是Li在算法OM(m-1)时收到的来自Lj的OralMsg, 利用 v=Majority(v1, v2...vi-1, vi+1,  vn-1)作为Li在OM(m)中作出的决定
//! 
//!
//! m > 0时, **Li从commander处收到的值不是立刻作为决定，而是发到网络里，收集其他节点的值来计算出Majrity值**。
//! 根据论文page 7的提示，OM(m)执行了 (n-1)(n-2)...(n-m)次OM(0)，按照这个公式来看，每一轮OM都会把前面的Commander排除掉，见`./res/om(3).png`
//! 也就是说要排除掉前面每一次出现的司令官。
//! 执行完OM(m-1)的第三步时，进入到OM(m)的第三步。根据算法第二步，算法OM(m)会产生n-1次OM(m-1)过程，
//! 那么此时第三步的共识阶段中, 每个Li会把自己在OM(m-1)中达成的n-1个值作为输入，放到Majority中产生输出。
//! 那么在 n > 3f 的保证下，忠诚节点相互传递的共识值v能够一直留存下来，称为最终共识值。
//! 
//! # 参考：
//! 1. The Byzantine Generals Problem. Leslie Lamport Robert Shostak Marshall Pease. ACM Transactions on Programming Languages and Systems | July 1982, pp. 382-401
//! 1. [拜占庭将军问题（二）——口头协议](https://www.jianshu.com/p/6591aea178fe)
//! 

use std::collections::{HashSet, HashMap};

type GeneralID = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Decision{
    Attack = 0, 
    // 副官如果收不到司令的命令，会采取撤退。
    Retrait = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct OralMsg{
    sender: GeneralID,
    m: usize,
    // None for sending nothing.
    msg: Option<Decision>,
}

fn get_msgs(self_id:GeneralID, m: usize, to_send:&HashSet<GeneralID>,
        val: OralMsg, is_traitor: bool) 
    -> HashMap<GeneralID, OralMsg>
{
    let mut hm = HashMap::new();
    for l in to_send{
        if is_traitor{
            let p = rand::random::<usize>() % 3;
            hm.insert(
                *l, 
                if p < 2{
                    OralMsg{
                        sender: self_id,
                        m: m,
                        msg: Some(
                        if p == 0 {
                            Decision::Attack
                        } else {
                            Decision::Retrait
                        }
                    )}
                }else{
                    OralMsg{sender: self_id, m: m, msg: None}
                }
            );
        }else{
            hm.insert(*l, val);
        }
    }
    hm
}


fn majority(self_id: GeneralID, vals: &mut Vec<OralMsg>) -> OralMsg{
    let l = vals.len();
    assert!(l > 0);
    let at_least = l >> 1;
    // 未收到的msg视为默认值
    vals.iter_mut().for_each(|x| 
        if x.msg.is_none(){
            x.msg = Some(Decision::Retrait);
        }
    );
    vals.sort_by_key(|a| a.msg.unwrap());
    let reach = Some(vals.get(at_least).unwrap().msg.unwrap());
    OralMsg{
        sender: self_id,
        m: vals[0].m,
        msg: reach,
    }
}

type MailBox = HashMap<(GeneralID, usize), Vec<OralMsg>>;

pub fn om(m: usize, commander:GeneralID, to_send: HashSet<GeneralID>, 
    vals: HashMap<GeneralID, OralMsg>, mb: &mut MailBox, is_traitor: &Vec<GeneralID>)
{
    eprintln!("om({}), cmdr={}...", m, commander);
    if m == 0{
        for l in &to_send{
            let om = vals.get(&l).unwrap();
            eprintln!("om({}), cmdr={} send {:?} to Li={}", m, commander, &om, l);
            let k = (*l, 0);
            let e = mb.entry(k).or_insert(vec![]);
            e.push(om.to_owned());
        }
        eprintln!("om(0), cmdr={} done...", commander);
        return ;
    }
    // m > 0
    // do step 1
    for l in &to_send{
        let msg = vals.get(&l).unwrap().clone();
        eprintln!("om({}), cmdr={} send {:?} to Li={}", m, commander, &msg, l);
        let e = mb.entry((*l, m-1)).or_insert(vec![]); // cmdr发来的val也要参与majority计算
        e.push(msg);
    }

    // do step 2
    for l in &to_send{
        let mut to_send_l = to_send.clone();
        let _ = to_send_l.remove(&l);
        let vals_l = get_msgs(
            *l, 
            m-1, &to_send_l, 
            vals.get(&l).unwrap().clone(), is_traitor.contains(&l)
        );
        om(m-1, l.clone(), to_send_l, vals_l, mb, is_traitor);
    }
    // do step 3
    for l in &to_send{
        let k = (*l, m-1);
        let decision = majority(*l, mb.get_mut(&k).unwrap());
        mb.remove(&k); // 释放空间OM(m-1)
        let e = mb.entry((*l, m)).or_insert(vec![]);
        e.push(decision);
        eprintln!("om({}, Li={} step 3 done... decision = {:?}", m, l, decision);
    }
}

#[test]
fn test_om() {
    fn random_traitor(n: usize, m: usize) -> Vec<GeneralID>{
        assert!(n >= 3 * m +1);
        let mut s = HashSet::new();
        loop{
            s.insert(rand::random::<GeneralID>() % n as GeneralID);
            if s.len() >= m{
                break
            }
        }
        s.into_iter().collect()
    }

    use std::iter::FromIterator;

    let m = 1;
    let n = 3 * m + 1;
    let commander = 1;
    let is_traitor = random_traitor(n, m);  // 叛徒名单
    let loyals = (0..n as GeneralID).filter(|x| !(is_traitor.contains(x))).collect::<Vec<GeneralID>>();
    let first_om = OralMsg{sender: commander, m: m as usize, msg: Some(Decision::Attack)};

    let mut mailboxs:MailBox = HashMap::new(); // 存放OM(m)的状态
    mailboxs.insert((commander, m), vec![first_om]); // 先放进去，后面的assertion 需要

    let mut to_send = HashSet::from_iter(0..n as GeneralID);

    to_send.remove(&commander);

    // loyal的将军原封不动地转发命令，叛徒随机发送消息。
    let vals = get_msgs(commander, m as usize, &to_send, first_om, is_traitor.contains(&commander));

    eprintln!("OM(m), for m = {}, n = {}:\ncommander = {}, traitor = {:?}, loyals = {:?}\nPS: None means sending nothing.", m, n, &commander, &is_traitor, &loyals);
    om(m as usize, commander, to_send, vals, &mut mailboxs, &is_traitor);

    eprintln!("id\tresult");
    for l in 0..n as GeneralID{
        eprintln!("{}\t{:?}", l, mailboxs.get(&(l, m)).unwrap()[0]);
    }

    // verify IC1
    let first = mailboxs.get(&(loyals[0], m)).unwrap()[0].msg;
    for l in loyals[1..].iter(){
        assert_eq!(first, mailboxs.get(&(*l, m))
                                    .unwrap()[0].msg);
    }

    // verify IC2
    if !is_traitor.contains(&commander){
        let first = mailboxs.get(&(commander, m)).unwrap()[0].msg;
        for l in loyals.iter(){
            assert_eq!(first, mailboxs.get(&(*l, m))
                                        .unwrap()[0].msg);
        }
    }
    


}