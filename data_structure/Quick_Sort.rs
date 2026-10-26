fn quick_sort(arr: &mut [i32]){
    let len = arr.len();
     
    if len <=1{
       return;
    }


   let pivot_index =partition(arr);
   quick_sort(&mut arr[..pivot_index]);
   quick_sort(&mut arr[pivot_index +1..]);


   fn partition(arr: &mut [i32] )  -> usize{
       let len =arr.len();
       let pivot =arr[len-1];
  

       let mut i =0;


       for j in 0..len-1{
           if arr[j]< pivot{
              arr.swap(i,j);
              i +=1;
           }
      }
      arr.swap(i,len-1);
      i
  }
}

fn main(){

    let mut arr =[7,8,20,1,4,3,2,67];
    quick_sort(&mut arr);
    println!("{:?}",arr);
}
