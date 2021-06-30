use crate::{ElPacket, Properties};
use core::fmt;
use phf::phf_map;
use serde::{Deserialize, Serialize};

pub enum ClassPacket {
    Unimplemented(UnimplementedPacket),
    StorageBattery(StorageBatteryPacket),
}

impl From<ElPacket> for ClassPacket {
    fn from(value: ElPacket) -> Self {
        match value.seoj.class {
            ClassCode(code::STORAGE_BATTERY) => ClassPacket::StorageBattery(value.into()),
            _ => ClassPacket::Unimplemented(value.into()),
        }        
    }
}

impl fmt::Display for ClassPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassPacket::StorageBattery(v) => write!(f, "{}", v)?,
            ClassPacket::Unimplemented(v) => write!(f, "{}", v)?,
        }
        Ok(())
    }
}

mod code {
    pub const STORAGE_BATTERY: [u8; 2] = [0x02, 0x7D];
    pub const CONTROLLER: [u8; 2] = [0x05, 0xFE];
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
struct ClassCode([u8; 2]);

impl fmt::Display for ClassCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X} {:02X}", self.0[0], self.0[1])
    }
}

static SUPER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0x80u8 => "動作状態",
    0x81u8 => "設置場所",
    0x82u8 => "規格version",
    0x83u8 => "識別番号",
    0x84u8 => "瞬時消費電力",
    0x85u8 => "積算消費電力",
    0x86u8 => "メーカ異常コード",
    0x87u8 => "電流制限設定",
    0x88u8 => "異常発生状態",
    0x89u8 => "異常内容",
    0x8Au8 => "メーカコード",
    0x8Bu8 => "事業場コード",
    0x8Cu8 => "商品コード",
    0x8Du8 => "製造番号",
    0x8Eu8 => "製造年月日",
    0x8Fu8 => "節電動作設定",
    0x93u8 => "遠隔操作設定",
    0x97u8 => "現在時刻設定",
    0x98u8 => "現在年月日設定",
    0x99u8 => "電力制限設定",
    0x9Au8 => "積算運転時間",
    0x9Du8 => "状変アナウンスプロパティマップ",
    0x9Eu8 => "Setプロパティマップ",
    0x9Fu8 => "Getプロパティマップ",
};

#[allow(dead_code)]
static PROFILE_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xBFu8 => "個体識別情報",
    0xD3u8 => "自ノードインスタンス数",
    0xD4u8 => "自ノードクラス数",
    0xD5u8 => "インスタンスリスト通知",
    0xD6u8 => "自ノードインスタンスリストS",
    0xD7u8 => "自ノードクラスリストS",
};

#[allow(dead_code)]
static HOUSEHOLD_SOLAR_POWER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xA0u8 => "出力制御設定１",
    0xA1u8 => "出力制御設定２",
    0xA2u8 => "余剰買取制御機能設定",
    0xB0u8 => "出力制御スケジュール",
    0xB1u8 => "次回アクセス日時",
    0xB2u8 => "余剰買取制御機能タイプ",
    0xB3u8 => "出力変化時間設定値",
    0xB4u8 => "上限クリップ設定値",
    0xC0u8 => "運転力率設定値",
    0xC1u8 => "FIT契約タイプ",
    0xC2u8 => "自家消費タイプ",
    0xC3u8 => "設備認定容量",
    0xC4u8 => "換算係数",
    0xD0u8 => "系統連系状態",
    0xD1u8 => "出力抑制状態",
    0xE0u8 => "瞬時発電電力計測値",
    0xE1u8 => "積算発電電力量計測値",
    0xE2u8 => "積算発電電力量リセット設定",
    0xE3u8 => "積算売電電力量計測値",
    0xE4u8 => "積算売電電力量リセット設定",
    0xE5u8 => "発電電力制限設定１",
    0xE6u8 => "発電電力制限設定２",
    0xE7u8 => "売電電力制限設定",
    0xE8u8 => "定格発電電力値（系統連系時",
    0xE9u8 => "定格発電電力値（独立時",
};

static STORAGE_BATTERY_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xA0u8 => "AC実効容量（充電）",
    0xA1u8 => "AC実効容量（放電）",
    0xA2u8 => "AC充電可能容量",
    0xA3u8 => "AC放電可能容量",
    0xA4u8 => "AC充電可能量",
    0xA5u8 => "AC放電可能量",
    0xA6u8 => "AC充電上限設定",
    0xA7u8 => "AC放電下限設定",
    0xA8u8 => "AC積算充電電力量計測値",
    0xA9u8 => "AC積算放電電力量計測値",
    0xAAu8 => "AC充電量設定値",
    0xABu8 => "AC放電量設定値",
    0xC1u8 => "充電方式",
    0xC2u8 => "放電方式",
    0xC8u8 => "最小最大充電電力値",
    0xC9u8 => "最小最大放電電力値",
    0xCAu8 => "最小最大充電電流値",
    0xCBu8 => "最小最大放電電流値",
    0xCCu8 => "再連系許可設定",
    0xCDu8 => "運転許可設定",
    0xCEu8 => "自立運転許可設定",
    0xCFu8 => "運転動作状態",
    0xC7u8 => "AC定格電力量",
    0xD0u8 => "定格電力量",
    0xD1u8 => "定格容量",
    0xD2u8 => "定格電圧",
    0xD3u8 => "瞬時充放電電力計測値",
    0xD4u8 => "瞬時充放電電流計測値",
    0xD5u8 => "瞬時充放電電圧計測値",
    0xD6u8 => "積算放電電力量計測値",
    0xD7u8 => "積算放電電力量リセット設定",
    0xD8u8 => "積算充電電力量計測値",
    0xD9u8 => "積算充電電力量リセット設定",
    0xDAu8 => "運転モード設定",
    0xDBu8 => "系統連系状態",
    0xDCu8 => "最小最大充電電力値（独立時）",
    0xDDu8 => "最小最大放電電力値（独立時）",
    0xDEu8 => "最小最大充電電流値（独立時）",
    0xDFu8 => "最小最大放電電流値（独立時）",
    0xE0u8 => "充放電量設定値1",
    0xE1u8 => "充放電量設定値2",
    0xE2u8 => "蓄電残量1",
    0xE3u8 => "蓄電残量2",
    0xE4u8 => "蓄電残量3",
    0xE5u8 => "劣化状態",
    0xE6u8 => "蓄電池タイプ",
    0xE7u8 => "充電量設定値1",
    0xE8u8 => "放電量設定値1",
    0xE9u8 => "充電量設定値2",
    0xEAu8 => "放電量設定値2",
    0xEBu8 => "充電電力設定値",
    0xECu8 => "放電電力設定値",
    0xEDu8 => "充電電流設定値",
    0xEEu8 => "放電電流設定値",
    0xEFu8 => "定格電圧（独立時）",
};

#[allow(dead_code)]
static EVPS_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xC0u8 => "車載電池の放電可能容量値1",
    0xC1u8 => "車載電池の放電可能容量値2",
    0xC2u8 => "車載電池の放電可能残容量1",
    0xC3u8 => "車載電池の放電可能残容量2",
    0xC4u8 => "車載電池の放電可能残容量3",
    0xC5u8 => "定格充電能力",
    0xC6u8 => "定格放電能力",
    0xC7u8 => "車両接続・充放電可否状態",
    0xC8u8 => "最小最大充電電力値",
    0xC9u8 => "最小最大放電電力値",
    0xCAu8 => "最小最大充電電流値",
    0xCBu8 => "最小最大放電電流値",
    0xCCu8 => "充放電器タイプ",
    0xCDu8 => "車両接続確認",
    0xCEu8 => "車載電池の充電可能容量値",
    0xCFu8 => "車載電池の充電可能残容量値",
    0xD0u8 => "車載電池の使用容量値1",
    0xD1u8 => "車載電池の使用容量値2",
    0xD2u8 => "定格電圧",
    0xD3u8 => "瞬時充放電電力計測値",
    0xD4u8 => "瞬時充放電電流計測値",
    0xD5u8 => "瞬時充放電電圧計測値",
    0xD6u8 => "積算放電電力量計測値",
    0xD7u8 => "積算放電電力量リセット設定",
    0xD8u8 => "積算充電電力量計測値",
    0xD9u8 => "積算充電電力量リセット設定",
    0xDAu8 => "運転モード設定",
    0xDBu8 => "系統連系状態",
    0xDCu8 => "充電方式",
    0xDDu8 => "放電方式",
    0xDEu8 => "買電電力設定値",
    0xDFu8 => "再連系許可設定",
    0xE2u8 => "車載電池の電池残容量1",
    0xE3u8 => "車載電池の電池残容量2",
    0xE4u8 => "車載電池の電池残容量3",
    0xE5u8 => "メンテナンス状態",
    0xE6u8 => "車両ID",
    0xE7u8 => "充電量設定値1",
    0xE9u8 => "充電量設定値2",
    0xEAu8 => "放電量設定値",
    0xEBu8 => "充電電力設定値",
    0xECu8 => "放電電力設定値",
    0xEDu8 => "充電電流設定値",
    0xEEu8 => "放電電流設定値",
    0xEFu8 => "定格電圧（独立時）",
};

pub struct UnimplementedPacket(ClassCode, Properties);
impl fmt::Display for UnimplementedPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Unimplemented Class: {}", self.0)?;
        for prop in self.1 .0.iter() {
            if let Some(name) = SUPER_CLASS.get(&prop.epc) {
                writeln!(f, "[{}]\t {}", name, prop)?;
                continue;
            }
            writeln!(f, "[unknown]\t {}", prop)?;
        }
        Ok(())
    }
}

impl From<ElPacket> for UnimplementedPacket {
    fn from(value: ElPacket) -> Self {
        UnimplementedPacket(value.seoj.class, value.props)
    }
}

pub struct StorageBatteryPacket(Properties);
impl StorageBatteryPacket {
    #[allow(dead_code)]
    const CODE: [u8; 2] = code::STORAGE_BATTERY;
}

impl fmt::Display for StorageBatteryPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "StorageBattery: 0x{:02X}{:02X}",
            Self::CODE[0],
            Self::CODE[1]
        )?;
        for prop in self.0 .0.iter() {
            if let Some(name) = SUPER_CLASS.get(&prop.epc) {
                writeln!(f, "[{}]\t {}", name, prop)?;
                continue;
            }
            if let Some(name) = STORAGE_BATTERY_CLASS.get(&prop.epc) {
                writeln!(f, "[{}]\t {}", name, prop)?;
            }
            writeln!(f, "[unknown]\t {}", prop)?;
        }
        Ok(())
    }
}

impl From<ElPacket> for StorageBatteryPacket {
    fn from(value: ElPacket) -> Self {
        if value.seoj.class != ClassCode(code::STORAGE_BATTERY) {
            panic!("source echonet object class must be storage battery.")
        }
        StorageBatteryPacket(value.props)
    }
}

pub struct Controller;
impl Controller {
    #[allow(dead_code)]
    const CODE: [u8; 2] = code::CONTROLLER;
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Controller: 0x{:02X}{:02X}",
            Self::CODE[0],
            Self::CODE[1]
        )
    }
}

enum Class {
    Controller(Controller),
}

impl From<ClassCode> for Class {
    fn from(code: ClassCode) -> Self {
        match code.0 {
            code::CONTROLLER => Class::Controller(Controller),
            _ => {
                todo!()
            }
        }
    }
}

impl From<EchonetObject> for Class {
    fn from(obj: EchonetObject) -> Self {
        Self::from(obj.class)
    }
}

// TODO: add methods
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct EchonetObject {
    class: ClassCode,
    instance: u8,
}

impl From<[u8; 3]> for EchonetObject {
    fn from(eobj: [u8; 3]) -> Self {
        Self {
            class: ClassCode([eobj[0], eobj[1]]),
            instance: eobj[2],
        }
    }
}

impl fmt::Display for EchonetObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {:02X}]", self.class, self.instance)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conversion() {
        let obj: EchonetObject = [0x05, 0xFE, 0x01].into();
        let _class: Class = obj.class.into();
        let _class: Class = obj.into();
    }
}
