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
//! 在算法的第三步：
//! ```
//! // 每一列都是某个节点
//!    L0 -> L1, L2, L3 with a
//! 
//!  OM(0)
//!    L
//!         L2 L3
//!    L1 = (a, a) = a
//!         L1 L3
//!    L2 = (a, a) = a
//!         L1 L2
//!    L3 = (a, a) = a
//! 
//! ```
//! 
//! # 参考：
//! 1. The Byzantine Generals Problem. Leslie Lamport Robert Shostak Marshall Pease. ACM Transactions on Programming Languages and Systems | July 1982, pp. 382-401
//! 1. [拜占庭将军问题（二）——口头协议](https://www.jianshu.com/p/6591aea178fe)
//! 

use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashSet;

type GeneralID = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Decision{
    Attack = 0, 
    // 副官如果收不到司令的命令，会采取撤退。
    Retrait = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct OralMsg{
    m: usize, 
    sender_id: GeneralID,
    // None for sending nothing.
    msg: Option<Decision>,
}

type MsgSender = Sender<OralMsg>;
type MsgReceiver = Receiver<OralMsg>;

pub struct General {
    self_id: GeneralID,
    others: Vec<GeneralID>,
    recvch: MsgReceiver,
    net: Box<dyn Router>,
    // 排除已经见过的将军
    visited: HashSet<GeneralID>,
}

impl General{
    #[inline]
    pub fn default_decision(&self) -> Decision{
        Decision::Retrait
    }
}

trait Router{
    fn send(&self, from: GeneralID, to:GeneralID, msg: OralMsg);

    // None means recv nothing.
    fn recv(&self, from: GeneralID, to:GeneralID) -> Option<OralMsg>;
}

struct SimpleRouter{

}

#[test]
fn test_solution(){
    // create generals
    let m = 1;
    let n = 3 * m + 1;
    let mut generals: Vec<General> = Vec::with_capacity(n);
    


}