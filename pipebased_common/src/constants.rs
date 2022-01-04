pub(crate) const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
pub(crate) const ID_LEN: usize = 12;
pub(crate) const DIRECTORY_CATALOGS: &str = "catalogs";
pub(crate) const SYSTEMD_DEFAULT_USER: &str = "pipebase";
pub(crate) const SYSTEMD_DEFAULT_GROUP: &str = "pipebase";
// https://www.freedesktop.org/wiki/Software/systemd/dbus/
pub(crate) const SYSTEMD_DEFAULT_START_UNIT_MODE: &str = "replace";
pub(crate) const SYSTEMD_DEFAULT_STOP_UNIT_MODE: &str = "replace";
pub(crate) const SYSTEMD_DEFAULT_DESCRIPTION: &str = "a pipebase application";
