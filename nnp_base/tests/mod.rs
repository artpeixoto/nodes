#[cfg(test)]
mod simple_test {
    extern crate nnp_base;
    extern crate std;

    use self::nnp_base::core::node::*;
    use self::nnp_base::core::proc::*;
    use self::nnp_base::runner::*;
    use std::marker::PhantomData;

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
            if args.deref() % 100_000_000 == 0 {
                println!("cycles count is {}", args.deref());
            }
        }
    }

    trait FunnyTrait<'a, TProc>
    where
        TProc: Process<'a, TArgs = NodeRef<'a, usize>> + 'a,
    {
    }

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

        for _ in 0..1_000_000_000 {
            runner.run_once();
        }

        let mut count = 0_u64;
        for _ in 0..1_000_000_000 {
            count += 1;
            if count % 100_000_000 == 0 {
                println!("cycles count is {}", count);
            }
        }
    }
    #[test]
    fn fuck() {
        let mut cycles_counter = CyclesCounter {};
        let mut cycles_count_logger = CyclesCountLogger {};

        type NodeIdKey = u16;
        #[derive(Clone, Copy, Default)]
        struct NodeId<const NODE_ID: NodeIdKey> {}
        impl<const NODE_ID: NodeIdKey> NodeId<NODE_ID> {
            fn new() -> Self {
                Self {}
            }
        }
        fn make_node_id<const NODE_ID: NodeIdKey>() -> NodeId<NODE_ID> {
            NodeId::new()
        }
        struct NodesNames {}
        impl NodesNames {
            pub const cycles_count: NodeIdKey = 0u16;
        }
        #[derive(Clone, Default)]
        pub struct NodesGetters {
            cycles_count: NodeId<{ NodesNames::cycles_count }>,
        }
        struct Nodes {
            cycles_count: Node<u64>,
        }
        impl OpensRef<NodeId<{ NodesNames::cycles_count }>> for Nodes {
            type TRet = Node<u64>;
            #[inline(always)]
            fn get_ref(&self, key: &NodeId<{ NodesNames::cycles_count }>) -> &Self::TRet {
                &self.cycles_count
            }
        }
        impl OpensMut<NodeId<{ NodesNames::cycles_count }>> for Nodes {
            #[inline(always)]
            fn get_mut(&mut self, key: &NodeId<{ NodesNames::cycles_count }>) -> &mut Self::TRet {
                &mut self.cycles_count
            }
        }
        type ProcIdKey = u16;
        #[derive(Clone, Copy, Default)]
        struct ProcId<const KEY: ProcIdKey> {}
        #[derive(Clone, Default)]
        struct ProcsGetters(pub ProcId<0u16>, pub ProcId<1u16>);
        struct Procs<TProc0, TProc1>(TProc0, TProc1)
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>;
        impl<TProc0, TProc1> OpensRef<ProcId<0u16>> for Procs<TProc0, TProc1>
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>,
        {
            type TRet = TProc0;
            #[inline(always)]
            fn get_ref(&self, key: &ProcId<0u16>) -> &Self::TRet {
                &self.0
            }
        }
        impl<TProc0, TProc1> OpensMut<ProcId<0u16>> for Procs<TProc0, TProc1>
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>,
        {
            #[inline(always)]
            fn get_mut(&mut self, key: &ProcId<0u16>) -> &mut Self::TRet {
                &mut self.0
            }
        }
        impl<TProc0, TProc1> OpensRef<ProcId<1u16>> for Procs<TProc0, TProc1>
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>,
        {
            type TRet = TProc1;
            #[inline(always)]
            fn get_ref(&self, key: &ProcId<1u16>) -> &Self::TRet {
                &self.1
            }
        }
        impl<TProc0, TProc1> OpensMut<ProcId<1u16>> for Procs<TProc0, TProc1>
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>,
        {
            #[inline(always)]
            fn get_mut(&mut self, key: &ProcId<1u16>) -> &mut Self::TRet {
                &mut self.1
            }
        }
        #[derive(Default)]
        struct NodesDependants {}
        impl NodesDependants {
            const cycles_count: &'static [ProcIdKey] = &[0u16, 1u16];
        }
        impl OpensRef<NodeId<{ NodesNames::cycles_count }>> for NodesDependants {
            type TRet = &'static [ProcIdKey];
            #[inline(always)]
            fn get_ref(&self, key: &NodeId<{ NodesNames::cycles_count }>) -> &Self::TRet {
                &NodesDependants::cycles_count
            }
        }
        struct RunnerData<TProc0, TProc1>
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>,
        {
            nodes: Nodes,
            nodes_getters: NodesGetters,
            nodes_dependants: NodesDependants,
            procs: Procs<TProc0, TProc1>,
            procs_getters: ProcsGetters,
        }
        #[derive(PartialEq, Eq, Clone)]
        enum ProcExecutionState {
            NotExecuted,
            Finished,
            ReQueued,
            Queued,
        }
        impl Default for ProcExecutionState {
            fn default() -> Self {
                ProcExecutionState::NotExecuted
            }
        }
        #[derive(Default)]
        struct CycleExecutionState(ProcExecutionState, ProcExecutionState);
        impl CycleExecutionState {
            fn clear(&mut self) {
                self.0 = ProcExecutionState::NotExecuted;
                self.1 = ProcExecutionState::NotExecuted
            }
            fn new() -> Self {
                let mut res = Self::default();
                res.clear();
                res
            }
        }
        impl OpensRef<ProcId<1u16>> for CycleExecutionState {
            type TRet = ProcExecutionState;
            
            #[inline(always)]

            fn get_ref(&self, key: &ProcId<1u16>) -> &Self::TRet {
                &self.1
            }
        }
        impl OpensMut<ProcId<1u16>> for CycleExecutionState {
            #[inline(always)]
            fn get_mut(&mut self, key: &ProcId<1u16>) -> &mut Self::TRet {
                &mut self.1
            }
        }
        impl OpensRef<ProcId<0u16>> for CycleExecutionState {
            type TRet = ProcExecutionState;
            fn get_ref(&self, key: &ProcId<0u16>) -> &Self::TRet {
                &self.0
            }
        }
        impl OpensMut<ProcId<0u16>> for CycleExecutionState {
            fn get_mut(&mut self, key: &ProcId<0u16>) -> &mut Self::TRet {
                &mut self.0
            }
        }
        struct CycleState {
            execution_queue: Deque<ProcIdKey, 2usize>,
            executed: CycleExecutionState,
        }
        impl CycleState {
            fn clear(&mut self) {
                self.execution_queue.clear();
                self.executed.clear();
            }
        }
        pub struct Runner<TProc0, TProc1>
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>,
        {
            current_cycle_state: CycleState,
            runner_data: RunnerData<TProc0, TProc1>,
        }
        impl<TProc0, TProc1> Runner<TProc0, TProc1>
        where
            TProc0: for<'a> Process<'a, TArgs = (<Node<u64> as TryDerefMut>::TMut<'a>)>,
            TProc1: for<'a> Process<'a, TArgs = (<Node<u64> as TryDeref>::TRef<'a>)>,
        {
            pub fn add_initial_procs(&mut self) {
                self.current_cycle_state
                    .execution_queue
                    .push_back(0u16)
                    .unwrap();
            }
            pub fn run_next_proc(&mut self) -> bool {
                match self.current_cycle_state.execution_queue.pop_front() {
                    Some(proc_id_key) => {
                        match proc_id_key {
                            0u16 => {
                                let proc_id = &self.runner_data.procs_getters.0;
                                let proc_execution_state =
                                    self.current_cycle_state.executed.get_mut(proc_id);
                                if proc_execution_state != &ProcExecutionState::Finished {
                                    let cycles_count_id =
                                        &self.runner_data.nodes_getters.cycles_count;
                                    match {
                                        let mut cycles_count = self
                                            .runner_data
                                            .nodes
                                            .get_ref(cycles_count_id)
                                            .try_deref_mut();

                                        (cycles_count)
                                    } {
                                        Ok((mut cycles_count)) => {
                                            let mut cycles_count_change_detector =
                                                ChangeDetector::new();
                                            cycles_count.add_change_detector(
                                                &mut cycles_count_change_detector,
                                            );
                                            self.runner_data
                                                .procs
                                                .get_mut(proc_id)
                                                .resume((cycles_count));
                                            *proc_execution_state = ProcExecutionState::Finished;
                                            if cycles_count_change_detector.has_changed() {
                                                {
                                                    let proc_id = &self.runner_data.procs_getters.0;
                                                    if self
                                                        .current_cycle_state
                                                        .executed
                                                        .get_ref(proc_id)
                                                        == &ProcExecutionState::NotExecuted
                                                    {
                                                        self.current_cycle_state
                                                            .execution_queue
                                                            .push_back(0u16)
                                                            .unwrap();
                                                        *(self
                                                            .current_cycle_state
                                                            .executed
                                                            .get_mut(proc_id)) =
                                                            ProcExecutionState::Queued;
                                                    }
                                                }
                                                {
                                                    let proc_id = &self.runner_data.procs_getters.1;
                                                    if self
                                                        .current_cycle_state
                                                        .executed
                                                        .get_ref(proc_id)
                                                        == &ProcExecutionState::NotExecuted
                                                    {
                                                        self.current_cycle_state
                                                            .execution_queue
                                                            .push_back(1u16)
                                                            .unwrap();
                                                        *(self
                                                            .current_cycle_state
                                                            .executed
                                                            .get_mut(proc_id)) =
                                                            ProcExecutionState::Queued;
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            if proc_execution_state == &ProcExecutionState::ReQueued
                                            {
                                                *proc_execution_state =
                                                    ProcExecutionState::Finished;
                                            } else {
                                                *proc_execution_state =
                                                    ProcExecutionState::ReQueued;
                                                self.current_cycle_state
                                                    .execution_queue
                                                    .push_back(0u16)
                                                    .unwrap();
                                            }
                                        }
                                    }
                                }
                            }
                            1u16 => {
                                let proc_id = &self.runner_data.procs_getters.1;
                                let proc_execution_state =
                                    self.current_cycle_state.executed.get_mut(proc_id);
                                if proc_execution_state != &ProcExecutionState::Finished {
                                    let cycles_count_id =
                                        &self.runner_data.nodes_getters.cycles_count;

                                    match {
                                        self.runner_data.nodes.get_ref(cycles_count_id).try_deref()
                                    } {
                                        Ok((cycles_count)) => {
                                            self.runner_data
                                                .procs
                                                .get_mut(proc_id)
                                                .resume((cycles_count));
                                            *proc_execution_state = ProcExecutionState::Finished;
                                        }
                                        Err(_) => {
                                            if proc_execution_state == &ProcExecutionState::ReQueued
                                            {
                                                *proc_execution_state =
                                                    ProcExecutionState::Finished;
                                            } else {
                                                *proc_execution_state =
                                                    ProcExecutionState::ReQueued;
                                                self.current_cycle_state
                                                    .execution_queue
                                                    .push_back(1u16)
                                                    .unwrap();
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                panic!("impossible shit");
                            }
                        };
                        true
                    }
                    None => false,
                }
            }
            pub fn run_forever(&mut self) -> ! {
                loop {
                    self.run_once();
                }
            }
            pub fn run_once(&mut self) {
                self.current_cycle_state.clear();
                self.add_initial_procs();
                loop {
                    if !self.run_next_proc() {
                        break;
                    }
                }
            }
        }

        let mut runner = Runner {
            current_cycle_state: CycleState {
                execution_queue: Deque::new(),
                executed: CycleExecutionState::new(),
            },
            runner_data: RunnerData {
                nodes: Nodes {
                    cycles_count: Node::from(0),
                },
                nodes_getters: NodesGetters::default(),
                nodes_dependants: NodesDependants {},
                procs: Procs(cycles_counter, cycles_count_logger),
                procs_getters: ProcsGetters::default(),
            },
        };
        for _ in 0..1_000_000_000 {
            runner.run_once();
        }

        let mut count = 0_u64;
        for _ in 0..1_000_000_000 {
            count += 1;
            if count % 100_000_000 == 0 {
                println!("cycles count is {}", count);
            }
        }
    }
}
