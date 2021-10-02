# 第六课

 ## 第一部分：为template模块添加Benchmark
为template模块的do_something添加benchmark用例，并且将benchmark运行结果转换为对应的权重定义；
## 在Cargo.toml添加依赖
打开 /pallets/template/Cargo.toml
```
vim /pallets/template/Cargo.toml
```
添加 sp-std 依赖
```
[dependencies.sp-std]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'monthly-2021-09+1'
version = '4.0.0-dev'
```
在 [features] 中 添加 sp-std/std
```
[features]
default = ['std']
runtime-benchmarks = ['frame-benchmarking']
std = [
 ...
'sp-std/std'
 ]
```
### 修改pallets
打开 /pallets/template/src/lib.rs
```
vim /pallets/template/src/lib.rs
```
找到 mod benchmarking; 在下面添加 pub mod weights;
```
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
```
找到 pub mod pallet ｛ 在下面添加 pub use crate::weights::WeightInfo;
```
pub mod pallet {
    pub use crate::weights::WeightInfo;
```
找到pub trait Config: frame_system::Config 在下面添加 type WeightInfo: WeightInfo;

```
#[pallet::config]
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    type WeightInfo: WeightInfo;
}
```
找到 pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult { 在上面添加
```
#[pallet::weight(T::WeightInfo::do_something(*something))]
pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
    ...
}
```
### 修改mock
打开 /pallets/template/src/mock/src.rs
找到 impl pallet_template::Config for Test,在下面添加 type WeightInfo
```
impl pallet_template::Config for Test {
	type Event = Event;
	type WeightInfo = pallet_template::weights::SubstrateWeight<Test>;
}
```
### 修改runtime
打开 /runtime/src/lib.rs
找到 impl  pallet_template::Config  for  Runtime { 在下面添加 type WeightInfo
```
impl pallet_template::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_template::weights::SubstrateWeight<Runtime>;
}
```
### 编译 benchmark
在node目录下运行
```
cd node
cargo build --release --features runtime-benchmarks
```
```
编译结果
substrate_lesson_homework_template/node-template-benchmark/runtime/Cargo.toml: version requirement `3.0.0-monthly-2021-09+1` for dependency `pallet-template` includes semver metadata which will be ignored, removing the metadata is recommended to avoid confusion
warning: /Users/edward/study/substrate_lesson_homework_template/node-template-benchmark/node/Cargo.toml: version requirement `3.0.0-monthly-2021-09+1` for dependency `node-template-runtime` includes semver metadata which will be ignored, removing the metadata is recommended to avoid confusion
    Updating git repository `https://github.com/paritytech/substrate.git`
    Finished release [optimized] target(s) in 2.17s

```
### 运行测试，并且生成weights.rs
在根目录下运行
 ```
 ./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_template --extrinsic do_something --steps 20 --repeat 50 --template=.maintain/frame-weight-template.hbs --output=./pallets/template/src/weights.rs
 ```
 ```
运行结果：
./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_template --extrinsic do_something --steps 20 --repeat 50 --template=.maintain/frame-weight-template.hbs --output=./pallets/template/src/weights.rs
Pallet: "pallet_template", Extrinsic: "do_something", Lowest values: [], Highest values: [], Steps: 20, Repeat: 50
Raw Storage Info
========
Storage: TemplateModule Something (r:0 w:1)

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=       23
    + s        0
              µs

Reads = 0 + (0 * s)
Writes = 1 + (0 * s)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    s   mean µs  sigma µs       %
    0     22.57     0.494    2.1%
    5     22.61     0.486    2.1%
   10     22.65     0.475    2.0%
   15     22.57     0.494    2.1%
   20     22.46     0.498    2.2%
   25        22         0    0.0%
   30     22.26     0.443    1.9%
   35      22.3     0.461    2.0%
   40     22.07     0.266    1.2%
   45        23         0    0.0%
   50      22.8     0.394    1.7%
   55        23         0    0.0%
   60        23         0    0.0%
   65        23         0    0.0%
   70     22.57     0.494    2.1%
   75     22.69     0.461    2.0%
   80        23         0    0.0%
   85     22.42     0.494    2.2%
   90        22         0    0.0%
   95     22.11     0.319    1.4%
  100     22.23     0.421    1.8%

Quality and confidence:
param     error
s             0

Model:
Time ~=    22.57
    + s        0
              µs

Reads = 0 + (0 * s)
Writes = 1 + (0 * s)
```

### 运行 test
```
cargo test -p pallet-template --all-features
```
```
warning: `pallet-template` (lib test) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 28.17s
     Running unittests (target/debug/deps/pallet_template-557c1073c84aa8a4)

running 4 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::it_works_for_default_value ... ok
test tests::correct_error_for_none_value ... ok
test benchmarking::benchmark_tests::test_benchmarks ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests pallet-template

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
### 重新编译 node-template
 ```
 cargo build --release
 ```
```
edward.wang node-template-benchmark % cargo build --release
warning: /Users/edward.wang/study/substrate_lesson_homework_template/node-template-benchmark/runtime/Cargo.toml: version requirement `3.0.0-monthly-2021-09+1` for dependency `pallet-template` includes semver metadata which will be ignored, removing the metadata is recommended to avoid confusion
warning: /Users/edward.wang/study/substrate_lesson_homework_template/node-template-benchmark/node/Cargo.toml: version requirement `3.0.0-monthly-2021-09+1` for dependency `node-template-runtime` includes semver metadata which will be ignored, removing the metadata is recommended to avoid confusion
    Updating git repository `https://github.com/paritytech/substrate.git`
    Finished release [optimized] target(s) in 2.78s
```
## 第二部分：生成ChainSpec文件

选择node-template或者其它节点程序，生成chain Spec 文件（两种格式都需要）:

### 生成customSpec.json文件

```
./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json

```
### 生成Node01和Node02密钥

 生成Node01 的 sr25519 密钥
```
subkey generate --scheme sr25519
```
```
Secret phrase:       coconut toilet monkey tenant tray arctic resemble blast law stick twenty black
  Secret seed:       0x78bc01b3314c56d59f55042be8b0de7f6e88b20684e979dcac0dc49dc96011f1
  Public key (hex):  0x1a54e06d0f3b2c3b8b025c5a07a0991128b07b0de4a8336acfb27effe7174b65
  Account ID:        0x1a54e06d0f3b2c3b8b025c5a07a0991128b07b0de4a8336acfb27effe7174b65
  Public key (SS58): 5CfEKGbjfx4vAX5MQ7Co7KjDg4oUsQe3HYixXkRvcEd2QwwV
  SS58 Address:      5CfEKGbjfx4vAX5MQ7Co7KjDg4oUsQe3HYixXkRvcEd2QwwV
```
生成 Node01 的 er25519 密钥
```
subkey inspect --scheme ed25519 "coconut toilet monkey tenant tray arctic resemble blast law stick twenty black"
```
```
Secret phrase:       coconut toilet monkey tenant tray arctic resemble blast law stick twenty black
  Secret seed:       0x78bc01b3314c56d59f55042be8b0de7f6e88b20684e979dcac0dc49dc96011f1
  Public key (hex):  0xc11896d44c85fe322321706c80e7307b6d758efd7550ca4112a022756024402e
  Account ID:        0xc11896d44c85fe322321706c80e7307b6d758efd7550ca4112a022756024402e
  Public key (SS58): 5GRtQMMg7KZxLddyt47vUsMLjW6t41XjZQjgbAjLL9Tfx4Mf
  SS58 Address:      5GRtQMMg7KZxLddyt47vUsMLjW6t41XjZQjgbAjLL9Tfx4Mf
```

生成Node02 的 sr25519 密钥
```
subkey generate --scheme sr25519
```
```
Secret phrase:       neck quit crowd mountain finish order hidden theory disease face theory nose
  Secret seed:       0xcaeb0b8886567f24a447b616b21fa0174912bc4fb32262619b7de0e8d9a777d3
  Public key (hex):  0x802358af7fed8c8353b3310c074be4dd5f3f432b16343ed6be3f77e7d4d28a3c
  Account ID:        0x802358af7fed8c8353b3310c074be4dd5f3f432b16343ed6be3f77e7d4d28a3c
  Public key (SS58): 5ExiVAnhSZ71Nhawh5ggixVj8DACyGhzpXnaWx72RekzsnrW
  SS58 Address:      5ExiVAnhSZ71Nhawh5ggixVj8DACyGhzpXnaWx72RekzsnrW
```
 生成Node02 的 er25519 密钥
```
subkey inspect --scheme ed25519 "neck quit crowd mountain finish order hidden theory disease face theory nose"
```
```
Secret phrase:       neck quit crowd mountain finish order hidden theory disease face theory nose
  Secret seed:       0xcaeb0b8886567f24a447b616b21fa0174912bc4fb32262619b7de0e8d9a777d3
  Public key (hex):  0x730adfd1bf7ded9ea8a04391fa359aab774f849b06746e00a26174c416aa1c89
  Account ID:        0x730adfd1bf7ded9ea8a04391fa359aab774f849b06746e00a26174c416aa1c89
  Public key (SS58): 5EfYbRHKCvZxt8SaGaewCLyWvnC4FHGsxPye2WLab5ut6cGa
  SS58 Address:      5EfYbRHKCvZxt8SaGaewCLyWvnC4FHGsxPye2WLab5ut6cGa
```
### 修改customSpec.json文件
```
vim customSpec.json
```
 - palletAura 修改为MyNode01,MyNode02 的sr25519 类型的SS58 Address
 - palletGrandpa 修改为MyNode01,MyNode02 的ed25519类型的SS58
 - Address sudo 修改为MyNode01 的sr25519 类型的SS58 Address
```
"genesis": {
	"palletAura": {
		"authorities":[
			"5DHcSekyAqJzzX3HcMdxBQGQdb9ZzV4FzERXmXvaTAHMKqPg",
			"5HHRM1NiusSjknFmDGKG25oudgdg4N4CfTdNimRMW1E3zqMU"
		]
	},
	"palletGrandpa": {
		"authorities": [
			["5CJAzvVYBjs8wYkVeEybdFGEkdbJJmtiQ5778uxAcKAisCUo", 1]
			["5HWccSqWwnCeRjw7JAfxp5RHEY33ZbiZG4YWJvXpkg8MaFd2", 1]
		]
	},
	...
	 "sudo": {
        "key": "5CfEKGbjfx4vAX5MQ7Co7KjDg4oUsQe3HYixXkRvcEd2QwwV"
      }
}
  ```

### 将修改后的 customSpec.json 转换为 "原生" chain spec
```
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```

## 第三部分：根据Chain Spec，部署公开测试网络

### Node01节点启动
```
./target/release/node-template purge-chain --base-path /tmp/node01 --chain ./customSpecRaw.json
./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'
```

### Node02节点启动
```
./target/release/node-template purge-chain --base-path /tmp/node02 --chain ./customSpecRaw.json
./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 31334 \
  --ws-port 19945 \
  --rpc-port 19933 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwVnwXSAnyhapwfB2AY9Zrz1t1f58BjbTv4qtFNCY5tQ \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'
```

### 上传Node01密钥
创建 node01_aura.json 文件：
```
vim node01_aura.json
```
修改内容,对应Node01的sr25519
```
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "aura",
    "coconut toilet monkey tenant tray arctic resemble blast law stick twenty black",
    "0x1a54e06d0f3b2c3b8b025c5a07a0991128b07b0de4a8336acfb27effe7174b65"
  ]
}
```
创建 node01_gran.json 文件：
```
vim node01_gran.json
```
修改内容,对应Node01的ed25519
```
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "gran",
    "coconut toilet monkey tenant tray arctic resemble blast law stick twenty black",
    "0xc11896d44c85fe322321706c80e7307b6d758efd7550ca4112a022756024402e"
  ]
}
```
上传到节点node01
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@/home/vico/substrate-node-template/node01_aura.json"
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@/home/vico/substrate-node-template/node01_gran.json"
```

### 上传Node02密钥
创建 node02_aura.json 文件：
```
vim node02_aura.json
```
修改内容,对应Node02的sr25519
```
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "aura",
    "neck quit crowd mountain finish order hidden theory disease face theory nose",
    "0x802358af7fed8c8353b3310c074be4dd5f3f432b16343ed6be3f77e7d4d28a3c"
  ]
}
```
创建 node02_gran.json 文件：
```
vim node02_gran.json
```
修改内容,对应Node02的ed25519
```
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "gran",
    "neck quit crowd mountain finish order hidden theory disease face theory nose",
    "0x730adfd1bf7ded9ea8a04391fa359aab774f849b06746e00a26174c416aa1c89"
  ]
}
```
上传到节点node02
```
curl http://localhost:19933 -H "Content-Type:application/json;charset=utf-8" -d "@/home/vico/substrate-node-template/node02_aura.json"
curl http://localhost:19933 -H "Content-Type:application/json;charset=utf-8" -d "@/home/vico/substrate-node-template/node02_gran.json"
```

