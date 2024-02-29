use crate::FileEntropy;

///
/// Struct for saving the Q1, Q3, and interquartile range data 
/// we use for outlier calculations. 
///
#[derive(Debug)]
pub struct IQR {
    pub q1: f64,
    pub q3: f64,
    pub iqr: f64
}

///
/// Calculate the mean of a [Vec] of [FileEntropy]
///
pub fn mean(entropies: &Vec<FileEntropy>) -> Option<f64> {
    // Return None if the set is empty
    if entropies.is_empty() {
        return None;
    }

    // Save the length for use in the calculation
    let entropies_len = entropies.len() as f64;
    Some(entropies.into_iter().map(|e| e.entropy).sum::<f64>() / entropies_len)
}

///
/// Sort a [Vec] of [FileEntropy] 
///
fn sort_entropies(entropies: &Vec<FileEntropy>) -> Vec<FileEntropy> {
    // We're gonna sort this thing, so copy it to work with it
    let mut sorted_entropies = entropies.clone();

    // Teach it how to sort
    // This is not chained because `sort_by()` does not return a new Vec
    sorted_entropies.sort_by(|a, b| a.entropy.partial_cmp(&b.entropy).unwrap());

    sorted_entropies
}

///
/// Calculate the median of a [Vec] of [FileEntropy]
///
pub fn median(entropies: &Vec<FileEntropy>) -> Option<f64> {
    // Return None if the set is empty
    if entropies.is_empty() {
        return None;
    }

    // We're gonna sort this thing, so copy it to work with it
    let sorted_entropies = sort_entropies(entropies);

    // Save length for easy use
    let len = sorted_entropies.len();

    // When even, median is the average of the two sides of the midpoint
    if len % 2 == 0 {
        let mid = len / 2;
        let median = (sorted_entropies[mid - 1].entropy + sorted_entropies[mid].entropy) / 2.0;
        Some(median)
    } else {
        let mid = len / 2;
        Some(sorted_entropies[mid].entropy)
    }
}

///
/// Calculate the [variance](https://en.wikipedia.org/wiki/Variance) of a [Vec] of [FileEntropy]
///
pub fn variance(entropies: &Vec<FileEntropy>) -> Option<f64> {
    // Return None if the set is empty
    if entropies.is_empty() {
        return None;
    }

    //  Save length for easy use (and passing borrow checking)
    let len = entropies.len() as f64;

    // Calculate the mean
    let mean = mean(entropies).unwrap();

    // Calculate the sum of squared differences from the mean
    let sum_of_squared_diffs = entropies
        .into_iter()
        .map(|e| (e.entropy - mean).powi(2))
        .sum::<f64>();

    // Calculate the variance
    let variance = sum_of_squared_diffs / len;

    Some(variance)
}

///
/// Calculate the [interquartile range](https://en.wikipedia.org/wiki/Interquartile_range) of a [Vec] of [FileEntropy]
/// We'll safe 
///
pub fn interquartile_range(entropies: &Vec<FileEntropy>) -> Option<IQR> {
    // Return None if the set is empty
    if entropies.is_empty() {
        return None;
    }

    // Sort the entropies in ascending order
    let sorted_entropies = sort_entropies(entropies);

    //  Save length for easy use (and passing borrow checking)
    let len = sorted_entropies.len();

    // Calculate the indices of the first quartile (Q1) and third quartile (Q3)
    // We have to account for even/odd weirdness with integers
    let q1_idx = match len % 2 {
        0 => len / 4,
        _ => (len + 1) / 4
    };

    // Multiply by 3 for Q3
    let q3_idx = 3 * q1_idx;

    // Calculate the values of the first quartile (Q1) and third quartile (Q3)
    let q1 = sorted_entropies[q1_idx - 1].entropy;
    let q3 = sorted_entropies[q3_idx - 1].entropy;
    // The IQR is the distance between the Q1 and Q3 values
    Some(IQR {
        iqr: q3 - q1,
        q1,
        q3
    })
}


///
/// Calculate outliers based on the IQR of the 
/// [Vec] of [FileEntropy]. 
///
pub fn entropy_outliers(entropies: &Vec<FileEntropy>) -> Option<Vec<FileEntropy>> {
    // Return None if the set is empty
    if entropies.is_empty() {
        return None;
    }

    // Unwrap is cool here because we solved the failure case above
    // But we clone here to handle the later move
    let iqr = interquartile_range(entropies).unwrap();
    let outliers: Vec<FileEntropy> = entropies
        .into_iter()
        .filter(|e| 
            e.entropy <= iqr.q1 - (1.5 * iqr.q1) || e.entropy >= iqr.q3 + (1.5 * iqr.iqr)
        )
        .map(|e| e.to_owned())
        .collect();

    // We do the user a solid and show the outliers sorted ascending
    Some(sort_entropies(&outliers))

}