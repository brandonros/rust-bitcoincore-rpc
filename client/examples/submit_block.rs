// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! A very simple example used as a self-test of this library against a Bitcoin
//! Core node.
extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, Error, RpcApi};
use bitcoincore_rpc_json::bitcoin::{block::{Version, Header}, BlockHash, hashes::Hash, hash_types::TxMerkleNode, CompactTarget, string::FromHexStr, absolute::LockTime, Transaction, Block, TxIn, TxOut, OutPoint, Txid, ScriptBuf, Sequence, Witness};

fn main_result() -> Result<(), Error> {
    let mut args = std::env::args();

    let _exe_name = args.next().unwrap();

    let url = args.next().expect("Usage: <rpc_url> <username> <password>");
    let user = args.next().expect("no user given");
    let pass = args.next().expect("no pass given");

    let rpc = Client::new(&url, Auth::UserPass(user, pass)).unwrap();

    let header = Header {
      version: Version::TWO,
      prev_blockhash: BlockHash::from_slice(&[0]).unwrap(), // TODO
      merkle_root: TxMerkleNode::from_slice(&[0]).unwrap(), // TODO
      time: 0, // TODO
      bits: CompactTarget::from_hex_str_no_prefix("00").unwrap(), // TODO
      nonce: 0, // TODO
    };
    
    let txdata = vec![
      Transaction { 
        version: 2, 
        lock_time: LockTime::from_time(0).unwrap(), // TODO
        input: vec![
          TxIn { 
            previous_output: OutPoint {
              txid: Txid::from_slice(&[0]).unwrap(), // TODO
              vout: 0 // TODO
            }, 
            script_sig: ScriptBuf::new(), // TODO
            sequence: Sequence::MAX,
            witness: Witness::new(), // TODO
          }
        ],
        output: vec![
          TxOut {
            value: 0, // TODO
            script_pubkey: ScriptBuf::new() // TODO
          }
        ]
      }
    ];

    let block = Block {
      header,
      txdata
    };
    
    rpc.submit_block(&block).unwrap();

    Ok(())
}

fn main() {
    main_result().unwrap();
}
