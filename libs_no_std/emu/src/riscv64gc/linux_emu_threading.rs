
pub struct LinuxEmuThreading {
    pub threads: Vec<CoreEmu>,
}

/// add yields before every mem read and after every mem write
/// to amplify data-races, the thread selection algorithm might change but
/// it has to be deterministic. 
/// To fuzz for data-races we can also re-run the same inputs but with multiple
/// scheduler seeds 