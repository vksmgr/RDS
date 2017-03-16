/*
//Saving the Universe


*//*
Problem:
The urban legend goes that if you go to the Google homepage and search for "Google", the universe will implode. We have a secret to share... It is true! Please don't try it, or tell anyone. All right, maybe not. We are just kidding.

The same is not true for a universe far far away. In that universe, if you search on any search engine for that search engine's name, the universe does implode!

To combat this, people came up with an interesting solution. All queries are pooled together. They are passed to a central system that decides which query goes to which search engine. The central system sends a series of queries to one search engine, and can switch to another at any time. Queries must be processed in the order they're received. The central system must never send a query to a search engine whose name matches the query. In order to reduce costs, the number of switches should be minimized.

Your task is to tell us how many times the central system will have to switch between search engines, assuming that we program it optimally.

*//*
pub fn save_the_universe() {
    let mut test_cases: i32 = 0;
    test_cases = read!();


    for current_test_case in 0..test_cases {
        //Local variables
        let mut no_serarch_engines: i32 = 0;
        let mut name_search_engine: Vec<String> = Vec::new();
        let mut no_incoming_querys: i32 = 0;
        let mut incoming_querys: Vec<String> = Vec::new();

        let mut count: i32 = 0;
        let mut min: i32 = 99999;
        let mut count: i32 = 0;


        //Get input


        no_serarch_engines = read!();
        for i in 0..(no_serarch_engines ) as usize {
            name_search_engine.push(read!("{}\n"));
        }

        no_incoming_querys = read!();
        for query in 0..(no_incoming_querys ) as usize {
            incoming_querys.push(read!("{}\n"));
        }

        //logic
        for mut index in 0..no_serarch_engines as usize {
            for j in 0..no_incoming_querys as usize {
                if name_search_engine[index] == incoming_querys[j] {
                    count = count + 1;
                    index = (index + 1) % no_serarch_engines as usize;
                }
                if (min > count) { min = count; }
            }
        }


        //output
        println!("Case #{}: {}",current_test_case+1, min);

    }
}*/
