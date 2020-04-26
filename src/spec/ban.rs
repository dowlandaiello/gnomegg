pub struct Ban {
    /// The ID of the user corresponding to this ban
    user_id: u64,

    /// The (optional) number of nanoseconds that this ban will be in effect for
    duration: Option<u64>,

    initiated_at: NaiveDateTime,

    ip: Option<String>
}
