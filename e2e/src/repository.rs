#[cfg(feature = "itest")]
#[cfg(test)]
mod tests {
    use crate::utils::build_client;
    use pipebased_common::grpc::daemon::{
        ListAppRequest, ListCatalogsRequest, PullAppRequest, PullCatalogsRequest, RemoveAppRequest,
        RemoveCatalogsRequest,
    };

    #[tokio::test]
    async fn test_repository() {
        let mut client = build_client("resources/cli.yml")
            .await
            .expect("build client failed");
        let resp = client
            .list_app(ListAppRequest {})
            .await
            .expect("list app failed")
            .into_inner();
        let apps = resp.apps;
        assert_eq!(0, apps.len());
        let resp = client
            .list_catalogs(ListCatalogsRequest {})
            .await
            .expect("list app failed")
            .into_inner();
        let catalogss = resp.catalogss;
        assert_eq!(0, catalogss.len());
        // test pull app
        client
            .pull_app(PullAppRequest {
                namespace: String::from("dev"),
                id: String::from("timer"),
                version: 0,
            })
            .await
            .expect("pull app failed");
        let resp = client
            .list_app(ListAppRequest {})
            .await
            .expect("list app failed")
            .into_inner();
        let apps = resp.apps;
        assert_eq!(1, apps.len());
        let app = apps.get(0).expect("no local app found");
        assert_eq!("dev", app.namespace);
        assert_eq!("timer", app.id);
        assert_eq!(0, app.version);
        // test pull catalogs
        client
            .pull_catalogs(PullCatalogsRequest {
                namespace: String::from("dev"),
                id: String::from("timer"),
                version: 0,
            })
            .await
            .expect("pull catalogs failed");
        let resp = client
            .list_catalogs(ListCatalogsRequest {})
            .await
            .expect("list app failed")
            .into_inner();
        let catalogss = resp.catalogss;
        assert_eq!(1, catalogss.len());
        let catalogs = catalogss.get(0).expect("no local catalogs found");
        assert_eq!("dev", catalogs.namespace);
        assert_eq!("timer", catalogs.id);
        assert_eq!(0, catalogs.version);
        // test remove app
        client
            .remove_app(RemoveAppRequest {
                namespace: String::from("dev"),
                id: String::from("timer"),
                version: 0,
            })
            .await
            .expect("remove app failed");
        let resp = client
            .list_app(ListAppRequest {})
            .await
            .expect("list app failed")
            .into_inner();
        let apps = resp.apps;
        assert_eq!(0, apps.len());
        // test remove catalogs
        client
            .remove_catalogs(RemoveCatalogsRequest {
                namespace: String::from("dev"),
                id: String::from("timer"),
                version: 0,
            })
            .await
            .expect("remove catalogs failed");
        let resp = client
            .list_catalogs(ListCatalogsRequest {})
            .await
            .expect("list app failed")
            .into_inner();
        let catalogss = resp.catalogss;
        assert_eq!(0, catalogss.len());
    }
}
