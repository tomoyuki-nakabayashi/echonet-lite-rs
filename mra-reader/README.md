# ECHONET Lite Machine Readable Appendix reader

Automatically generates property maps used in echonet-lite crate with phf::Map format.

## How to use

Download and unzip the JSON machine readable appendix from ECHONET Lite consortium's web page.

https://echonet.jp/spec_mra_rp1/

```shell
$ cargo run -- <path to MRA>/MRA_V1.1.1/mraData/devices/0x0130.json
pub static homeAirConditioner: phf::Map<u8, &'static str> = phf_map! {
    0x80u8 => "動作状態",
    0x8Fu8 => "節電動作設定",
    0x90u8 => "ONタイマ予約設定",
    0x91u8 => "ONタイマ時刻設定値",
    0x92u8 => "ONタイマ相対時間設定値",
    0x94u8 => "OFFタイマ予約設定",
    0x95u8 => "OFFタイマ時刻設定値",
    0x96u8 => "OFFタイマ相対時間設定値",
    0xA0u8 => "風量設定",
    0xA0u8 => "風量設定",
    0xA1u8 => "風向自動設定",
    0xA3u8 => "風向スイング設定",
    0xA4u8 => "風向上下設定",
    0xA5u8 => "風向左右設定",
    0xAAu8 => "特殊状態",
    0xABu8 => "非優先状態",
    0xB0u8 => "運転モード設定",
    0xB1u8 => "温度自動設定",
    0xB2u8 => "急速動作モード設定",
    0xB3u8 => "温度設定値",
    0xB4u8 => "除湿モード時相対湿度設定値",
    0xB5u8 => "冷房モード時温度設定値",
    0xB6u8 => "暖房モード時温度設定値",
    0xB7u8 => "除湿モード時温度設定値",
    0xB8u8 => "定格消費電力値",
    0xB9u8 => "消費電流計測値",
    0xBAu8 => "室内相対湿度計測値",
    0xBBu8 => "室内温度計測値",
    0xBBu8 => "室内温度計測値",
    0xBCu8 => "ユーザリモコン温度設定値",
    0xBDu8 => "吹き出し温度計測値",
    0xBEu8 => "外気温度計測値",
    0xBFu8 => "相対温度設定値",
    0xC0u8 => "換気モード設定",
    0xC1u8 => "加湿モード設定",
    0xC2u8 => "換気風量設定",
    0xC4u8 => "加湿量設定",
    0xC6u8 => "搭載空気清浄方法",
    0xC7u8 => "空気清浄機能モード設定",
    0xC8u8 => "搭載リフレッシュ方法",
    0xC9u8 => "リフレッシュ機能モード設定",
    0xCAu8 => "搭載自己洗浄方法",
    0xCBu8 => "自己洗浄機能モード設定",
    0xCCu8 => "特別運転モード設定",
    0xCDu8 => "内部動作状態",
    0xCEu8 => "強制サーモモード設定",
    0xCEu8 => "強制サーモモード設定",
    0xCFu8 => "空気清浄モード設定",
    0xD0u8 => "ブザー",
};
```

## how to test

Please install cargo make if you have not installed yet.

```shell
$ cargo install cargo-make
```

```shell
$ cargo make test-mra-reader
```
