use std::io::Read;
use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian as LE};

use serde::Serialize;

use bincode;

use flate2::{GzBuilder, Compression};

#[derive(Copy, Clone)]
pub struct Target {
    pub target: u64,
    pub hover_target: u64,
    pub focus_target: u64,
}

impl Target {
    #[allow(dead_code)]
    pub fn from_ffxiv_slice<D: AsRef<[u8]>>(slice: D) -> Target {
        Target::try_from_ffxiv_slice(slice).expect("Unable to read target data")
    }

    pub fn try_from_ffxiv_slice<D: AsRef<[u8]>>(slice: D) -> std::io::Result<Target> {
        let slice = slice.as_ref();
        let mut cursor = Cursor::new(slice);
        Ok(()).and_then(|_| {
            cursor.set_position(192);
            let target = cursor.read_u64::<LE>()?;
            let hover_target = cursor.read_u64::<LE>()?;
            cursor.set_position(280);
            let focus_target = cursor.read_u64::<LE>()?;

            Ok(Target { target, focus_target, hover_target })
        })
    }

    #[allow(dead_code)]
    pub fn as_ffxiv_array(&self) -> [u8; 512] {
        Ok::<(), Box<dyn std::error::Error>>(()).and_then(|_| {
            let mut buffer = [0u8; 512];
            let mut cursor = Cursor::new(buffer.as_mut());
            cursor.set_position(192);
            cursor.write_u64::<LE>(self.target)?;
            cursor.write_u64::<LE>(self.hover_target)?;
            cursor.set_position(280);
            cursor.write_u64::<LE>(self.focus_target)?;
            Ok(buffer)
        }).expect("Couldn't write target")


    }
}

#[derive(Serialize)]
pub struct Combatant {
    pub name: [u8; 30],
    pub id: u32,
    pub bnpcid: u32,
    pub ownerid: u32,
    pub tipe: u8,
    pub effective_distance: u8,
    pub pos_x: f32,
    pub pos_z: f32,
    pub pos_y: f32,
    pub heading: f32,
    pub pctargetid: u32,
    pub npctargetid: u32,
    pub bnpcnameid: u32,
    pub current_world_id: u16,
    pub home_world_id: u16,
    pub current_hp: u32,
    pub max_hp: u32,
    pub current_mp: u32,
    pub max_mp: u32,
    pub current_gp: u16,
    pub max_gp: u16,
    pub current_cp: u16,
    pub max_cp: u16,
    pub job: u8,
    pub level: u8,
    pub is_casting_1: u8,
    pub is_casting_2: u8,
    pub cast_buff_id: u32,
    pub cast_duration_current: f32,
    pub cast_duration_max: f32,
}

impl Combatant {
    #[allow(dead_code)]
    pub fn from_slice<D: AsRef<[u8]>>(slice: D) -> Combatant {
        Combatant::try_from_slice(slice).expect("Unable to read combatant")
    }

    pub fn try_from_slice<D: AsRef<[u8]>>(slice: D) -> std::io::Result<Combatant> {
        let mut cursor= Cursor::new(slice);
        Ok(())
            .and_then(|_| {

                let id: u32;
                let bnpcid: u32;
                let ownerid: u32;
                let tipe: u8;
                let effective_distance: u8;
                let pos_x: f32;
                let pos_z: f32;
                let pos_y: f32;
                let heading: f32;
                let pctargetid: u32;
                let npctargetid: u32;
                let bnpcnameid: u32;
                let current_world_id: u16;
                let home_world_id: u16;
                let current_hp: u32;
                let max_hp: u32;
                let current_mp: u32;
                let max_mp: u32;
                let current_gp: u16;
                let max_gp: u16;
                let current_cp: u16;
                let max_cp: u16;
                let job: u8;
                let level: u8;
                let is_casting_1: u8;
                let is_casting_2: u8;
                let cast_buff_id: u32;
                let cast_duration_current: f32;
                let cast_duration_max: f32;

                let mut name_buffer = [0u8; 30];
                cursor.set_position(48);
                cursor.read_exact(&mut name_buffer)?;
                cursor.set_position(116);
                id = cursor.read_u32::<LE>()?;
                cursor.set_position(128);
                bnpcid = cursor.read_u32::<LE>()?;
                cursor.set_position(132);
                ownerid = cursor.read_u32::<LE>()?;
                cursor.set_position(140);
                tipe = cursor.read_u8()?;
                cursor.set_position(146);
                effective_distance = cursor.read_u8()?;
                cursor.set_position(160);
                pos_x = cursor.read_f32::<LE>()?;
                pos_z = cursor.read_f32::<LE>()?;
                pos_y = cursor.read_f32::<LE>()?;
                cursor.set_position(176);
                heading = cursor.read_f32::<LE>()?;
                cursor.set_position(1000);
                pctargetid = cursor.read_u32::<LE>()?;
                cursor.set_position(6176);
                npctargetid = cursor.read_u32::<LE>()?;
                cursor.set_position(6268);
                bnpcnameid = cursor.read_u32::<LE>()?;
                cursor.set_position(6296);
                current_world_id = cursor.read_u16::<LE>()?;
                home_world_id = cursor.read_u16::<LE>()?;
                cursor.set_position(6308);
                current_hp = cursor.read_u32::<LE>()?;
                max_hp = cursor.read_u32::<LE>()?;
                current_mp = cursor.read_u32::<LE>()?;
                max_mp = cursor.read_u32::<LE>()?;
                cursor.set_position(6326);
                current_gp = cursor.read_u16::<LE>()?;
                max_gp = cursor.read_u16::<LE>()?;
                current_cp = cursor.read_u16::<LE>()?;
                max_cp = cursor.read_u16::<LE>()?;
                cursor.set_position(6364);
                job = cursor.read_u8()?;
                cursor.set_position(6366);
                level = cursor.read_u8()?;
                cursor.set_position(7248);
                is_casting_1 = cursor.read_u8()?;
                cursor.set_position(7250);
                is_casting_2 = cursor.read_u8()?;
                cursor.set_position(7252);
                cast_buff_id = cursor.read_u32::<LE>()?;
                cursor.set_position(7300);
                cast_duration_current = cursor.read_f32::<LE>()?;
                cast_duration_max = cursor.read_f32::<LE>()?;

                Ok(Combatant {
                    name: name_buffer,
                    id,
                    bnpcid,
                    ownerid,
                    tipe,
                    effective_distance,
                    pos_x,
                    pos_z,
                    pos_y,
                    heading,
                    pctargetid,
                    npctargetid,
                    bnpcnameid,
                    current_world_id,
                    home_world_id,
                    current_hp,
                    max_hp,
                    current_mp,
                    max_mp,
                    current_gp,
                    max_gp,
                    current_cp,
                    max_cp,
                    job,
                    level,
                    is_casting_1,
                    is_casting_2,
                    cast_buff_id,
                    cast_duration_current,
                    cast_duration_max
                })

            })
    }

    pub fn binary_serialize_compressed(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        let cursor = Cursor::new(&mut ret);

        let mut gz = GzBuilder::new()
            .write(cursor, Compression::default());
        bincode::serialize_into(&mut gz, &self).expect("Unable to serialize combatant");
        gz.finish().expect("Unable to compress combatant");
        ret
    }



}

#[cfg(test)]
mod models_tests {
    use crate::mem::models::Combatant;
    use std::io::{Cursor, Write};

    #[test]
    fn combatant_serialize() {
        let a = Combatant::from_slice(&[0u8; 7308].as_ref());
        let mut compress_vec = Vec::new();
        let cursor = Cursor::new(&mut compress_vec);
        let mut g = flate2::write::GzEncoder::new(cursor, flate2::Compression::default());
        g.write_all(bincode::serialize(&a).unwrap().as_ref()).unwrap();
        g.finish().unwrap();
        assert_eq!(compress_vec.as_slice(), a.binary_serialize_compressed().as_slice());
    }
}