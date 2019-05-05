#[macro_use]
extern crate jsonrpc_client_core;
extern crate byteorder;
extern crate ckb_core;
extern crate ckb_protocol;
extern crate ckb_script;
extern crate jsonrpc_client_http;
extern crate jsonrpc_types;
extern crate numext_fixed_hash;

use jsonrpc_client_http::HttpTransport;
use std::convert::TryFrom;
use std::env;
use std::str::FromStr;

use ckb_core::transaction::{
    CellInput as CoreCellInput, CellOutput as CoreCellOutput, Transaction as CoreTransaction,
};
use jsonrpc_types::Transaction;
use numext_fixed_hash::H256;

use ckb_protocol::Script as FbsScript;
use ckb_protocol::{
    Bytes as FbsBytes, CellInputBuilder, CellOutput as FbsCellOutput, OutPoint as FbsOutPoint,
};

use ckb_script::build_tx;

use flatbuffers::FlatBufferBuilder;

use std::fs::{create_dir, OpenOptions};
use std::io;
use std::io::prelude::*;

use byteorder::{LittleEndian, WriteBytesExt};

jsonrpc_client!(pub struct CKBClient {
    pub fn get_transaction(&mut self, _hash: H256) -> RpcRequest<Transaction>;
});

fn write_file(filename: &str, data: &[u8]) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .create_new(true)
        .append(true)
        .open(filename);

    match file {
        Ok(mut stream) => {
            stream.write_all(data)?;
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    Ok(())
}

fn main() -> ::std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need only one argument : tx hash (without 0x prefix)!");
    }
    let tx_hash = H256::from_str(&args[1]).unwrap();

    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle("http://localhost:8114").unwrap();
    let mut client = CKBClient::new(transport_handle);
    let tx = client.get_transaction(tx_hash).call().unwrap();
    let mut dep_cells: Vec<CoreCellOutput> = vec![];
    for pointer in tx.clone().deps {
        let tx = client.get_transaction(pointer.hash).call().unwrap();
        let core_tx: CoreTransaction = ckb_core::transaction::Transaction::try_from(tx).unwrap();
        let dep_cell = core_tx.outputs()[pointer.index as usize].clone();
        dep_cells.push(dep_cell);
    }
    let mut input_cells: Vec<CoreCellOutput> = vec![];
    for input in tx.clone().inputs {
        let tx = client
            .get_transaction(input.previous_output.hash)
            .call()
            .unwrap();
        let core_tx: CoreTransaction = ckb_core::transaction::Transaction::try_from(tx).unwrap();
        let input_cell = core_tx.outputs()[input.previous_output.index as usize].clone();
        input_cells.push(input_cell);
    }

    // from rpc types to core types
    let core_tx: CoreTransaction = ckb_core::transaction::Transaction::try_from(tx).unwrap();
    let outputs_cells: Vec<&CoreCellOutput> = core_tx.outputs().iter().collect();
    let inputs: Vec<&CoreCellInput> = core_tx.inputs().iter().collect();

    // write files
    create_dir("data")?;
    let mut tx_builder = FlatBufferBuilder::new();
    let tx_offset = build_tx(&mut tx_builder, &core_tx);
    tx_builder.finish(tx_offset, None);
    let tx_data = tx_builder.finished_data();

    // for ckb_load_tx
    write_file("data/tx", &tx_data)?;

    // for ckb_load_cell
    /*
    enum Source {
        Current = 0,
        Input = 1,
        Output = 2,
        Dep = 3,
    }
    */
    for (i, cell) in dep_cells.iter().enumerate() {
        let mut builder = FlatBufferBuilder::new();
        let offset = FbsCellOutput::build(&mut builder, cell);
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/cell_{}_3", i);
        write_file(&filename, &data)?;
    }
    for (i, cell) in outputs_cells.iter().enumerate() {
        let mut builder = FlatBufferBuilder::new();
        let offset = FbsCellOutput::build(&mut builder, cell);
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/cell_{}_2", i);
        write_file(&filename, &data)?;
    }
    for (i, cell) in input_cells.iter().enumerate() {
        let mut builder = FlatBufferBuilder::new();
        let offset = FbsCellOutput::build(&mut builder, cell);
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/cell_{}_1", i);
        write_file(&filename, &data)?;
    }

    // for ckb_load_cell_by_field
    /*
    enum CellField {
        Capacity = 0,
        Data = 1,
        DataHash = 2,
        Lock = 6,
        LockHash = 3,
        Type = 4,
        TypeHash = 5,
    }
    */
    for (i, cell) in dep_cells.iter().enumerate() {
        // field capacity
        let mut buffer = vec![];
        buffer.write_u64::<LittleEndian>(cell.capacity)?;
        let filename = format!("data/cell_field_{}_3_0", i);
        write_file(&filename, &buffer)?;
        // field data
        let filename = format!("data/cell_field_{}_3_1", i);
        write_file(&filename, &cell.data)?;
        // field data hash
        let hash = cell.data_hash();
        let bytes = hash.as_bytes();
        let filename = format!("data/cell_field_{}_3_2", i);
        write_file(&filename, &bytes)?;
        // field lock hash
        let hash = cell.lock.hash();
        let bytes = hash.as_bytes();
        let filename = format!("data/cell_field_{}_3_3", i);
        write_file(&filename, &bytes)?;
        // field type
        if let Some(ref type_) = cell.type_ {
            let mut builder = FlatBufferBuilder::new();
            let offset = FbsScript::build(&mut builder, &type_);
            builder.finish(offset, None);
            let data = builder.finished_data();
            let filename = format!("data/cell_field_{}_3_4", i);
            write_file(&filename, &data)?;
        };
        // filed type hash
        if let Some(ref type_) = cell.type_ {
            let hash = type_.hash();
            let bytes = hash.as_bytes();
            let filename = format!("data/cell_field_{}_3_5", i);
            write_file(&filename, &bytes)?;
        };
        // filed lock
        let mut builder = FlatBufferBuilder::new();
        let offset = FbsScript::build(&mut builder, &cell.lock);
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/cell_field_{}_3_6", i);
        write_file(&filename, &data)?;
    }

    for (i, cell) in outputs_cells.iter().enumerate() {
        // field capacity
        let mut buffer = vec![];
        buffer.write_u64::<LittleEndian>(cell.capacity)?;
        let filename = format!("data/cell_field_{}_2_0", i);
        write_file(&filename, &buffer)?;
        // field data
        let filename = format!("data/cell_field_{}_2_1", i);
        write_file(&filename, &cell.data)?;
        // field data hash
        let hash = cell.data_hash();
        let bytes = hash.as_bytes();
        let filename = format!("data/cell_field_{}_2_2", i);
        write_file(&filename, &bytes)?;
        // field lock hash
        let hash = cell.lock.hash();
        let bytes = hash.as_bytes();
        let filename = format!("data/cell_field_{}_2_3", i);
        write_file(&filename, &bytes)?;
        // field type
        if let Some(ref type_) = cell.type_ {
            let mut builder = FlatBufferBuilder::new();
            let offset = FbsScript::build(&mut builder, &type_);
            builder.finish(offset, None);
            let data = builder.finished_data();
            let filename = format!("data/cell_field_{}_2_4", i);
            write_file(&filename, &data)?;
        };
        // filed type hash
        if let Some(ref type_) = cell.type_ {
            let hash = type_.hash();
            let bytes = hash.as_bytes();
            let filename = format!("data/cell_field_{}_2_5", i);
            write_file(&filename, &bytes)?;
        };
        // filed lock
        let mut builder = FlatBufferBuilder::new();
        let offset = FbsScript::build(&mut builder, &cell.lock);
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/cell_field_{}_2_6", i);
        write_file(&filename, &data)?;
    }

    for (i, cell) in input_cells.iter().enumerate() {
        // field capacity
        let mut buffer = vec![];
        buffer.write_u64::<LittleEndian>(cell.capacity)?;
        let filename = format!("data/cell_field_{}_1_0", i);
        write_file(&filename, &buffer)?;
        // field data
        let filename = format!("data/cell_field_{}_1_1", i);
        write_file(&filename, &cell.data)?;
        // field data hash
        let hash = cell.data_hash();
        let bytes = hash.as_bytes();
        let filename = format!("data/cell_field_{}_1_2", i);
        write_file(&filename, &bytes)?;
        // field lock hash
        let hash = cell.lock.hash();
        let bytes = hash.as_bytes();
        let filename = format!("data/cell_field_{}_1_3", i);
        write_file(&filename, &bytes)?;
        // field type
        if let Some(ref type_) = cell.type_ {
            let mut builder = FlatBufferBuilder::new();
            let offset = FbsScript::build(&mut builder, &type_);
            builder.finish(offset, None);
            let data = builder.finished_data();
            let filename = format!("data/cell_field_{}_1_4", i);
            write_file(&filename, &data)?;
        };
        // filed type hash
        if let Some(ref type_) = cell.type_ {
            let hash = type_.hash();
            let bytes = hash.as_bytes();
            let filename = format!("data/cell_field_{}_1_5", i);
            write_file(&filename, &bytes)?;
        };
        // filed lock
        let mut builder = FlatBufferBuilder::new();
        let offset = FbsScript::build(&mut builder, &cell.lock);
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/cell_field_{}_1_6", i);
        write_file(&filename, &data)?;
    }

    // for ckb_load_input_by_field
    // only for input cells
    for (i, input) in inputs.iter().enumerate() {
        // field args
        let mut builder = FlatBufferBuilder::new();
        let vec = input
            .args
            .iter()
            .map(|argument| FbsBytes::build(&mut builder, argument))
            .collect::<Vec<_>>();
        let args = builder.create_vector(&vec);
        // Since a vector cannot be root FlatBuffer type, we have
        // to wrap args here inside a CellInput struct.
        let mut input_builder = CellInputBuilder::new(&mut builder);
        input_builder.add_args(args);
        let offset = input_builder.finish();
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/input_field_{}_1_0", i);
        write_file(&filename, &data)?;
        // field outpoint
        let mut builder = FlatBufferBuilder::new();
        let offset = FbsOutPoint::build(&mut builder, &input.previous_output);
        builder.finish(offset, None);
        let data = builder.finished_data();
        let filename = format!("data/input_field_{}_1_1", i);
        write_file(&filename, &data)?;
    }

    Ok(())
}
