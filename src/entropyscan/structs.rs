use std::borrow::Cow;
use std::path::PathBuf;

use tabled::Tabled;

#[derive(Debug, Clone)]
pub struct FileEntropy {
    pub path: PathBuf,
    pub entropy: f64,
}

impl Tabled for FileEntropy {

    const LENGTH: usize = 2;

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::from("PATH".to_string()),
            Cow::from("ENTROPY".to_string())
        ]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::from(self.path.to_str().unwrap()),
            Cow::from(format!("{:.3}", self.entropy))
        ]
    }
}

///
/// Contains our statistics information 
/// for easy reporting and display. 
///
#[derive(Debug, Clone)]
pub struct Stats {
    pub target: PathBuf,
    pub total: usize,
    pub mean: f64,
    pub median: f64,
    pub variance: f64,
}

impl Tabled for Stats {

    const LENGTH: usize = 4;

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::from("PATH".to_string()),
            Cow::from("TOTAL".to_string()),
            Cow::from("MEAN".to_string()),
            Cow::from("MEDIAN".to_string()),
            Cow::from("VARIANCE".to_string())
        ]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::from(self.target.to_str().unwrap()),
            Cow::from(format!("{}", self.total)),
            Cow::from(format!("{:.3}", self.mean)),
            Cow::from(format!("{:.3}", self.median)),
            Cow::from(format!("{:.3}", self.variance))
        ]
    }
}