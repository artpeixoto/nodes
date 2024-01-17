type DurationType  		= fixed::types::I40F24;
type SmallDurationType 	= fixed::types::I10F22; // apenas para 1024 sec ou menos. 
type TimeType      		= fixed::types::U40F24;

pub type Duration 	= DurationType;
pub type Time 		= TimeType;

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Duration ( pub fixed::types::I40F24 );

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Time     ( pub fixed::types::U40F24) ;

// impl Into<Time> for f32{
//     fn into(self) -> Time {
// 		Time(self.into())
//     }
// }
// impl From<Time> for f32{
//     fn from(self) -> Time {
// 		Time(self.into())
//     }
// }

// impl Sub<Duration> for Time{
//     type Output = Time;

//     fn sub(self, rhs: Duration) -> Self::Output {
//         Time(self.0 - rhs.0)
//     }
// }

// impl Add<Duration> for Time{
//     type Output = Time;
//     fn add(self, rhs: Duration) -> Self::Output {
// 		let sum_res = self.0 + rhs.0 ;
// 		Time(sum_res)
//     }
// }

// impl Add<Duration> for Duration{
//     type Output = Duration;
//     fn add(self, rhs: Time) -> Self::Output {
// 		let sum_res = self.0 + rhs.0 ;
// 		Duration(sum_res)
//     }
// }

// impl<TNum: Num> Mul<TNum> for Duration {
//     type Output = Duration;
//     fn mul(self, rhs: TNum) -> Self::Output {
// 		Duration(rhs * self.0)
//     }
// }

// impl Sub<Duration> for Duration{
//     type Output = Duration;

//     fn sub(self, rhs: Duration) -> Self::Output {
// 		Duration(self.0 - rhs.0)
//     }
// }

