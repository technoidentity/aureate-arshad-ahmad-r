fn merge(arr: &mut [i32]){
    let len = arr.len();

        if len <=1{
           return;
        }

    let mid = len/2;

    merge(&mut arr[..mid]);
    merge(&mut arr[mid..]);

    let mut temp =arr.to_vec();
    let (mut i, mut j, mut k) = (0,mid,0);


    while i< mid && j< len{
        if arr[i] <= arr[j]{
            temp[k]=arr[i];
            i+=1;
        }else {
            temp[k]= arr[j];
            j +=1;
        }
        k +=1;
    }
    while i< mid{
        temp[k] = arr[i];
        i +=1;
        k +=1;
    }
    while j< len{
        temp[k]= arr[j];
        j +=1;
        k +=1;
    }
    arr.copy_from_slice(&temp)
}

fn main(){
        let mut arr =[ 5,8,9,3,2,7,1];
        merge(&mut arr);
        println!("{:?}",arr)
}
