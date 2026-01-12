
fn main() {
    csvtotable("Jan_2026_weather.csv", 2);
    //let data_array = csvtotable("Jan 2026 weather.csv");
    //println!("data array looks like {:?}", data_array); // what is the {:?} operator?
}



//function to import data to an array
fn csvtotable(filename: &str, previewlimit:usize) /*-> Vec<usize>*/{ // what is the type Vec<usize>?
    /* 
    let readcsv = csv::Reader::from_path(filename); //Implementation of Reader from csv dependency
    */
    let readcsv = csv::ReaderBuilder::new().has_headers(false).from_path(filename); //Implementation of ReaderBuilder from csv dependency
    let readcsv_clone = csv::ReaderBuilder::new().has_headers(false).from_path(filename); //Clone: This is my solution for counting the number of rows for now.
    
    println!("successfully uploaded {filename}");
    let mut readcsv_t = readcsv.unwrap();
    let mut readcsv_tclone = readcsv_clone.unwrap();
    
    
    
    let mut col_amount: usize= 0;
    let mut row_amount: usize= 0;
    for dp in readcsv_tclone.records() {
        if row_amount == 0 {
            col_amount = dp.unwrap().len()
        }
        row_amount += 1;
    }

    println!("length of readcsv_t.records() (This is the number of rows in the CSV file): {}", row_amount);
    println!("readcsv_t.records().len() (This is the number of columns in the CSV file): {}", col_amount);


    // The following block of code is to preview the first "previewlimit" rows in the CSV file   
    let mut i: usize = 0; //why not u8?
    for dp in readcsv_t.records() {    //what are records()? Returns a ?(borrowed iterator) over all ?records as strings. Also seems to return the row in the CSV file.
                                                                    //The data type of dp is unknown-- This question can be answered once for loop is understood in Rust.
                                                                        //Data type of x in "for x in y" is the specific element in y where the current loop is. i.e y[current loop].  
        
        if i < previewlimit {
            println!("for loop ran");
            println!("dp is: {:#?}", dp);
            let dp_itm = dp.unwrap(); 
            println!("unwrapped dp is: {:#?}", dp_itm);
            println!("check if dp_itm.count() counts the number of columns: {}",dp_itm.len());
            let dp_itm1 = dp_itm.get(0);
            println!("the zeroth element is {:#?}",dp_itm1);
            let dp_itm2 = dp_itm1.unwrap();
            println!("unwrapped zeroth element is {}", dp_itm2);
            println!("i is: {}", i)
        }    
        i += 1;
    }
    println!("total iterations: {}", i);
    // The preceding block of code is to preview the first "previewlimit" rows in the CSV file
    


}

