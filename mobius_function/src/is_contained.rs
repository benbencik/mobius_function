pub fn is_contained(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    /*
    Naive implementation of a function that checks if
    multipermutation "a" is contained in "b"

    Assusmes that "a" has |b| or |b-1| elements
    */

    let n: usize = b.len();
    let mut valid;

    // check for assumption
    if b.len() < a.len() {
        return false;
    }
    if (b.len() - a.len()) > 1 {
        return false;
    }

    if a.len() == b.len() {
        // |a| == |b|
        valid = true;
        for j in 0..n - 1 {
            // itterate through all positions
            for k in j + 1..n {
                if (a[j] == a[k] && b[j] != b[k]) || (a[j] <= a[k] && b[j] > b[k]) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            return true;
        }
    } else {
        // |a| < |b|
        for i in 0..n {
            // choose omitted element
            valid = true;
            '_first_index: for j in 0..n - 2 {
                // itterate through all positions
                '_second_index: for k in j + 1..n - 1 {
                    let elem1 = if j < i { j } else { j + 1 };
                    let elem2 = if k < i { k } else { k + 1 };
                    if (a[j] == a[k] && b[elem1] != b[elem2])
                        || (a[j] <= a[k] && b[elem1] > b[elem2])
                    {
                        valid = false;
                        break '_first_index;
                    }
                }
            }
            if valid {
                return true;
            }
        }
    }
    return false;
}
