/// How_to_generate_all_sets_of_P_packages_into_N_bags
/// 
/// Author: Joao Nuno Carvalho
/// Date:   2022.01.01
/// 
/// Santa Claus has to optimize the space for everyone present on the sled.
/// 
/// Description: Santa Claus got him self a quantum computer, but the computer
///              is still being designed by a big high tech Open Source and Open
///              Hardware company (it's a donation to the common good for 2022).
///              So Santa asked a bunch of developers to make small pieces of
///              code to help him Optimize the placement of all presents in the
///              sled bags. The algorithm that was attributed to me was a simple
///              one:
/// 
///    * Generate all sets of P packages into N bags
/// 
/// Others will have to make other parts of all the Christmas software,
/// already preparing for the christmas of 2022 ...
/// 
/// This is a nice way to start the new year, looking and developing some code
/// for Santa Claus.
/// 
/// How does it work? : Well, we will do a backtracking algorithm with recurrence.
///                     In it we will transverse the stack, implicitly drawing a
///                     tree (depth first traversal) that at each node as a state
///                     for each configuration.
///                     We will be making changes to the state has we go down the
///                     tree and we will undo does changes has we go back up the tree.
///                     In the first node we will put all the presents in the first
///                     bag and then we will shift the presents between bags
///                     (from left to right) into all places, into all combinations
///                     as we traverse the tree. At each node we test the constrains
///                     and collect the new presents in the bags configuration.
/// 
/// 
/// Tree
/// 
///                                     (A,B,C:_:_)
///                      ____________________||____________________
///                  (A,B:C:_)                                 (A,B:_:C)
///            __________||__________                    __________||__________
///        (A:B,C:_)             (A:C:B)              (A:B:C)            (A:_:B,C)
///       _____||_____         _____||_____         _____||_____        _____||_____
/// (_:A,B,C:_) (_:B,C:A)  (_:A,C:B) (_:C:A,B)  (_:A,B:C) (_:B:A,C)  (_:A:B,C) (_:_:A,B,C)
/// 
/// 
/// How many states are there?
/// 
/// 
///                Max P
///                -----
///                \             p 
///  Num states =   |     (b - 1)
///                /
///                -----
///                 p=0
/// 
/// 
///  Where:
///     "Max P" is the number of Presents or Packages.
///     b is the number of Bags in the Sled.
/// 
/// 
/// License: MIT Open Source License
/// 
/// 


// A list of all the possible States.
type AllSates = Vec<State>;

fn main() {
    println!("\n");
    println!("***************************************************");
    println!("**  Generate all sets of p packages into n bags  **");
    println!("***************************************************");

    let num_packages = 3_usize;
    let list_packages = vec!['A', 'B', 'C'];
    println!("\nnum packages: {}   {:?}", num_packages, list_packages);
    let num_bags = 3_usize;
    println!("num bags:     {}", num_bags);

    // Create the bags of packages.
    let mut all_states: AllSates = vec![];
    // Fill the first state with the first bag with all the packages.
    let mut curr_state = State::new(num_bags);
    curr_state.set_bag(0, list_packages.clone());
    curr_state.print_state(false);
    let mut counter = 1_usize;
    solve(& mut counter, & mut curr_state, & mut all_states);
    println!("\nAll possible states:");
    for state in & all_states {
        let flag_sort = true;
        state.print_state(flag_sort);
    }
    println!("\nnum states: {}", calculate_num_states(num_packages, num_bags));
    println!("\nnum valid states: {}", all_states.len());
    assert_eq!(counter, all_states.len());
}

/// A State is composed of n bags that can old any number of packages.
struct State {
    st: Vec<Vec<char>>,
    num_bags: usize
}

impl State {
    fn new(num_bags: usize) -> Self {
        let mut st = Vec::with_capacity(num_bags);
        for _ in 0..num_bags {
           st.push(vec!['_']); 
        }
        State{
            st: st,
            num_bags
        }
    }

    fn clone_it(& self) -> Self {
        let mut st_clone = Vec::with_capacity(self.st.len());
        for bag in & self.st {
            st_clone.push(bag.clone());
        }
        State{
            st: st_clone,
            num_bags: self.num_bags
        }
    }

    fn set_bag(& mut self, num_bag: usize, packages_vec: Vec<char>) {
        self.st[num_bag] = packages_vec;
    }

    fn shift_last_from_bags(& mut self, start_pos: usize, end_pos: usize) -> bool {
        if self.st[start_pos][0] == '_' {
            return false;
        }
        let c = self.st[start_pos].pop().unwrap();
        if self.st[start_pos].len() == 0 {
            self.st[start_pos].push('_');
        }
        if self.st[end_pos][0] == '_' {
            self.st[end_pos].pop();        
        }
        self.st[end_pos].push(c);
        true
    }

    fn print_state(& self, flag_sort: bool) {
        print!("( ");
        for (index_a, bag) in self.st.iter().enumerate() {
            let mut bag = bag.clone();
            if flag_sort {
                bag.sort();
            }
            for (index_b, package) in bag.iter().enumerate() {
                print!("{}", package);
                if index_b != bag.len() - 1 {
                    print!(", ");
                }
            }
            if index_a != self.st.len() - 1 {
                print!(" : ");
            }
        }
        println!(" )");
    }
    
    fn first_bag_is_empty(& self) -> bool {
        self.st[0][0] == '_'
    }
}

fn is_constrain_satisfied(state: & State) -> bool {
    // NOTE: Add here your conditions for constrains of each state.
    true
}

fn solve(counter: & mut usize, curr_state: & mut State , all_states: & mut AllSates) {

    if all_states.len() == 0 {
        all_states.push(curr_state.clone_it());
    }

    // Stop condition
    if curr_state.first_bag_is_empty() {
        return;
    }

    let mut end_pos= 1000_usize;
    for i in 1..curr_state.num_bags {
        let start_pos = i - 1;
        end_pos = i;
        curr_state.shift_last_from_bags(start_pos, end_pos);        
        if is_constrain_satisfied(& curr_state) {
            *counter += 1_usize;
            all_states.push(curr_state.clone_it());
        }
        solve(counter, curr_state, all_states);
    }
    // Removes the modifications of the current state.
    let start_pos = end_pos;
    end_pos = 0;
    curr_state.shift_last_from_bags(start_pos, end_pos);
}

fn calculate_num_states(num_packages: usize, num_bags: usize) -> usize {
    let mut acc: i64 = 0;
    let packages = num_packages as u32;
    let b = num_bags as i64;    
    for p in 0..(packages + 1) {
        
        acc += i64::pow(b - 1, p );
    }
    acc as usize
}


