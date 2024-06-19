use core::{default, mem};
use core::ops::{Deref, DerefMut};
use crate::base::node::*;

#[derive(PartialEq, Eq, Default)]
pub enum Request<TReq, TAns> {
    #[default]
    StandingBy,
    Question(TReq),
    Processing,
    Answer(TAns),
}


impl<TQues, TAns> Request<TQues, TAns> {
    pub unsafe fn put_question_unchecked(&mut self, question: TQues){
        *self = Request::Question(question);
    }
    pub const fn is_standing_by(&self) -> bool{
        if let Request::StandingBy = self{
            true
        } else {
            false
        }
    }
    pub fn put_question( self: &mut impl DerefMut<Target=Self>, question: TQues) -> Result<(), TQues> {
        if self.is_standing_by() {
            unsafe{ self.put_question_unchecked(question);}
            Ok(())
         } else {
            Err(question)
        }
    }
    pub const fn peek_question(&self) -> Option<&TQues>{
        if let Request::Question(question) = self{
            Some(question)
        } else {
            None
        }
    }
    pub const fn peek_answer(&self) -> Option<&TAns>{
        if let Request::Answer(ans) = self{
            Some(ans)
        } else {
            None
        }
    }

    pub const fn is_question(&self) -> bool{
        match self {
            Request::Question(_) => true,
            _ => false,
        }
    }
    
    pub const fn is_waiting(&self) -> bool{
        match self{
            Request::Processing => true,
            _ => false
        }
    }
    pub fn take_question(self: &mut impl DerefMut<Target=Self>) -> Option<TQues>{
        if self.is_question(){
            Some(unsafe {self.deref_mut().take_question_unchecked()})
        } else {
            None
        }
    }
    pub unsafe fn take_question_unchecked(&mut self) -> TQues {
        let Request::Question(question) =  mem::replace(self, Request::Processing) else {panic!()};
        question
    }
    
    pub fn take_answer(mut self: &mut impl DerefMut<Target=Self>) -> Option<TAns>{
        if self.is_answer(){
            Some(unsafe{self.deref_mut().take_answer_unchecked()})
        } else{
            None
        }
    } 
    pub fn take_answer_if(mut self: &mut impl DerefMut<Target=Self>, predicate: impl FnOnce(&TAns) -> bool) -> Option<TAns>{
        if self.peek_answer().is_some_and(predicate){
            Some(unsafe{self.deref_mut().take_answer_unchecked()})
        } else {
            None
        }
    } 
    pub unsafe fn take_answer_unchecked(&mut self) -> TAns{
        let Request::Answer(ans) = mem::replace(self, Request::StandingBy) else {panic!()};
        ans
    }
    pub fn put_answer(mut self: &mut impl DerefMut<Target=Self>, answer: TAns) -> Result<(), TAns>{
        if self.is_waiting(){
            unsafe{self.deref_mut().put_answer_unchecked(answer)}
            Ok(())
        } else {
            Err(answer)
        }
    }
    pub fn put_answer_unchecked(&mut self, answer: TAns) {
        *self = Request::Answer(answer);
    }

    pub const fn is_answer(&self) -> bool{
        match self{
            Request::Answer(_) => {true}
            _ => false
        }
    }
}

pub type RequestNode<TReq, TAns> = Node<Request<TReq, TAns>>;
pub type RequestNRef<'a, TReq, TAns> = NodeRef<'a, Request<TReq, TAns>>;
pub type RequestNMut<'a, TReq, TAns> = NodeRefMut<'a, Request<TReq, TAns>>;
