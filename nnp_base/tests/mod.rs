#[cfg(test)]
mod simple_test {
    extern crate std;
    extern crate nnp_base;

    use std::marker::PhantomData;
    use self::nnp_base::core::node::*;
    use self::nnp_base::core::proc::*;
    use self::nnp_base::runner::*;

    struct CyclesCounter {}

    impl Process for CyclesCounter {
        type TArgs<'a> = NodeRefMut<'a, u64>;
        fn resume<'a>(&mut self, mut args: Self::TArgs<'a>) {
            *args += 1_u64;
        }
    }

    struct CyclesCountLogger {}

    impl Process for CyclesCountLogger {
        type TArgs<'a> = NodeRef<'a, u64>;
        fn resume<'a>(&mut self, args: Self::TArgs<'a>) {
            if args.deref() % 10_000_000 == 0 {
                println!("cycles count is {}", args.deref());
            }
        }
    }

    trait FunnyTrait<'a, TProc> where TProc: Process<TArgs<'a> = NodeRef<'a, usize>> + 'a {}

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