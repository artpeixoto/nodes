#[cfg(test)]
mod simple_test {
    extern crate std;
    extern crate nnp_base;

    use std::marker::PhantomData;
    use self::nnp_base::core::node::*;
    use self::nnp_base::core::proc::*;
    use self::nnp_base::runner::*;

    struct CyclesCounter {}

    impl<'a> Process<'a> for CyclesCounter {
        type TArgs = NodeRefMut<'a, u64>;
        fn resume(&mut self, mut args: Self::TArgs) {
            *args += 1_u64;
        }
    }

    struct CyclesCountLogger {}

    impl<'a> Process<'a> for CyclesCountLogger {
        type TArgs = NodeRef<'a, u64>;
        fn resume(&mut self, args: Self::TArgs) {
            if args.deref() % 1_000_000_000 == 0 {
                println!("cycles count is {}", args.deref());
            }
        }
    }

    trait FunnyTrait<'a, TProc> where TProc: Process<'a, TArgs = NodeRef<'a, usize>> + 'a {}

    struct FunnyRock<'a> {
        _phantom: PhantomData<&'a ()>,
    }

    pub fn sample<T>(el: T) {}

    #[test]
    pub fn test() {
        let mut cycles_counter = CyclesCounter {};
        let mut cycles_count_logger = CyclesCountLogger {};

        build_runner!(
            nodes: {
                cycles_count: Node<u64> = 0
            },
            processes: {
                cycles_counter(mut cycles_count)!,
                cycles_count_logger(cycles_count)
            }
        );

        runner.run_forever();
    }
}