
pub fn run(){
    prob();
}


/*

Problem

You receive a credit C at a local store and would like to buy two items. You first walk through the store and
create a list L of all available items. From this list you would like to buy two items that add up to the entire
value of the credit. The solution you provide will consist of the two integers indicating the positions of the
items in your list (smaller number first).

Input

The first line of input gives the number of cases, N. N test cases follow. For each test case there will be:

One line containing the value C, the amount of credit you have at the store.
One line containing the value I, the number of items in the store.
One line containing a space separated list of I integers. Each integer P indicates the price of an item in the store.
Each test case will have exactly one solution.
Output

For each test case, output one line containing "Case #x: " followed by the indices of the two items whose price
adds up to the store credit. The lower index should be output first.

*/
fn prob(){
    let mut t: i8 = read!();
    let mut credits: i32 = 0;
    let mut item: i32 = 0;
    let mut array: [i32; 2000] = [1; 2000];
    let mut flag: bool = false;
    for case in 0..t {

        credits = read!();
        item = read!();

        for i in 0..item as usize {
            array[i] = read!();
        }
        let half = item /2;
        for v in 0..item as usize {
            flag = false;
            for b in 0..item as usize {
                if array[v] + array[b] == credits && v != b {
                    print!("Case #{}: {} {}",case+1, v+1, b+1);
                    flag = true;
                }

            }
            if flag{
                break;
            }
        }
        println!("");
    }

   
}