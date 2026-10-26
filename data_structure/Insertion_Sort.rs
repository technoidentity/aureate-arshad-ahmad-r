fn insertion(arr : &mut [i32]){
     let len = arr.len();


     for i in 1..len{
        let key =arr[i];
        let mut j=i;
    
     while j>0 && arr[j-1] >key{
         arr[i] = arr[j-1];
         j-=1;
     }
     arr[j]=key;
     }
}

fn main(){
    let mut arr =[87,45,34,89,42,12,23,36];
    insertion(&mut arr);
    println!("{:?}",arr)
}
