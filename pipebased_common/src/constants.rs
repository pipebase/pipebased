pub(crate) const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
pub(crate) const ID_LEN: usize = 12;
pub(crate) const PATH_APP: &str = "app";
pub(crate) const PATH_APP_LOCK: &str = "app.lock";
pub(crate) const PATH_APP_REGISTER: &str = "app.reg";
pub(crate) const PATH_CATALOGS: &str = "catalogs";
pub(crate) const PATH_CATALOGS_REGISTER: &str = "catalogs.reg";
pub(crate) const PATH_CATALOGS_LOCK: &str = "catalogs.lock";
pub(crate) const SYSTEMD_DEFAULT_USER: &str = "pipebase";
pub(crate) const SYSTEMD_DEFAULT_GROUP: &str = "pipebase";
// https://www.freedesktop.org/wiki/Software/systemd/dbus/
pub(crate) const SYSTEMD_DEFAULT_START_UNIT_MODE: &str = "replace";
pub(crate) const SYSTEMD_DEFAULT_STOP_UNIT_MODE: &str = "replace";
pub(crate) const SYSTEMD_DEFAULT_DESCRIPTION: &str = "a pipebase application";
