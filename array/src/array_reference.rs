pub fn array_reference (){
    let arr : [i32; 5] = [10, 20, 30, 40, 50];
    copy_array(arr);
    println!("Array after function call: {:?}", arr); // Original array remains unchanged

    let mut arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array before reference function call: {:?}", arr2);
    reference_array(&mut arr2); // Passing a reference to the array
    println!("Array after reference function call: {:?}", arr2); // Array reflects changes made through the reference
}

fn copy_array(arr1: [i32; 5]) {
    let mut arr2 = arr1; // Copying the array
    arr2[0] = 100; // Modifying the copied array
    println!("Original array: {:?}", arr1); // Original array remains unchanged
    println!("Modified copied array: {:?}", arr2); // Copied array reflects the change
}

fn reference_array(arr:&mut [i32; 5]) {
    println!("Array via reference: {:?}", arr);
    arr[0] = 200; // This line would cause a compile-time error because we cannot modify through an immutable reference
    println!("Modified array via reference: {:?}", arr);
}