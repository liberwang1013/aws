fn main() {
    let _rsp = aws::logs::create_export_task("/prod-bj/core/lb-gateway-auth",
                                            1565452800,
                                            1565539200,
                                            "prod-lb-logs",
                                            "prod-bj/core/lb-gateway-auth");
}
