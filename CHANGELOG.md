# 1.0.0 (2025-01-02)


### Bug Fixes

* `Cord.override` should be of lowest priority ([a12c353](https://github.com/vyfor/cord.nvim/commit/a12c3536433ab80e9e7a0480461a78d4b3b0a33a))
* ABI compatibility issues on Apple Silicon due to long argument lists in functions ([db805fa](https://github.com/vyfor/cord.nvim/commit/db805fadd45561ad1cc781cadb77554843eaae9f))
* c & csharp displaying incorrectly ([b300334](https://github.com/vyfor/cord.nvim/commit/b30033472c68aa2c548468030cb34b002d852504))
* change backgrounds of the file icons to a solid color to avoid inconsistencies ([3ce1569](https://github.com/vyfor/cord.nvim/commit/3ce15695178966e213522c8aae19c25f906fc896))
* check for license file's file extension ([b1da43c](https://github.com/vyfor/cord.nvim/commit/b1da43c0e6b910f4a1b54d4ef9a62e01aa82eb36))
* connection issues ([a0729c1](https://github.com/vyfor/cord.nvim/commit/a0729c1c940f84d83744a0c75182ba4802bed2ca))
* correct param typedef in update presence function ([991f9f5](https://github.com/vyfor/cord.nvim/commit/991f9f57d5ecf6d12c3a3ce1bc79be1cd9b2043a))
* correct param typedef in update presence function ([299cd3e](https://github.com/vyfor/cord.nvim/commit/299cd3edba48d005b747dfd9dfa4e1f1d869242c))
* correct the order of arguments ([fc05cb3](https://github.com/vyfor/cord.nvim/commit/fc05cb39fa76aa1c0992afc14a3c596b4bd0a416))
* correct the toggleterm lua pattern ([6e1e62a](https://github.com/vyfor/cord.nvim/commit/6e1e62ae0ff22b89bb4516983db411be8a817ab3))
* correctly parse git repository's url ([7d422eb](https://github.com/vyfor/cord.nvim/commit/7d422ebf0051d79e39211c26cb27a6e7253fc7de))
* correctly parse git ssh urls ([7925b31](https://github.com/vyfor/cord.nvim/commit/7925b31544413a220015f11556f92066d2bd8e32))
* Correctly parse Git SSH urls ([#53](https://github.com/vyfor/cord.nvim/issues/53)) ([c366ac0](https://github.com/vyfor/cord.nvim/commit/c366ac0e7b3a8fb2b854a21fa53e279ba5e89403))
* correctly parse toggleterm executable names ([518e9b8](https://github.com/vyfor/cord.nvim/commit/518e9b8544c2d9a77208091d0e613a353002cd5b))
* disabling show_repository also hides the other buttons ([edcea53](https://github.com/vyfor/cord.nvim/commit/edcea532920c4678d03c23ed97617a161f9a29fa))
* displaying incorrect column number ([41c5089](https://github.com/vyfor/cord.nvim/commit/41c508965c17f372c305ff7c90e544bfcef0085d))
* enforce that `last_presence.idle` is `true` when idling ([d99adc6](https://github.com/vyfor/cord.nvim/commit/d99adc65160b30c247c875ca6cbc4f746afe978c))
* fallback to default icons if no client id was provided ([d0cb15a](https://github.com/vyfor/cord.nvim/commit/d0cb15a1eff7516515da6eef06f5589cd643035a))
* Fallback to default icons if no client ID was provided ([7cea2c3](https://github.com/vyfor/cord.nvim/commit/7cea2c3fe0bb3b9045492f64021b1911c8fb3964))
* fallback to file extensions on unknown filetypes ([c0d1dd1](https://github.com/vyfor/cord.nvim/commit/c0d1dd15201a7563d6ee1af755915a10af70a1cb))
* fallback to the default image, in case one was not provided along with the client id ([46435f1](https://github.com/vyfor/cord.nvim/commit/46435f165e38212a2190bd1fe4f6eebd86c93a1f))
* focus event resetting the idle status variable ([356b0b0](https://github.com/vyfor/cord.nvim/commit/356b0b0ed0e1b488f4e07492f65340e5397d7e66))
* forgot to rename the files ([50d602d](https://github.com/vyfor/cord.nvim/commit/50d602da06f1b2757c12872ca210d6b9ab46722d))
* handle workspace blacklists in rust ([34bc8db](https://github.com/vyfor/cord.nvim/commit/34bc8dbbf96f999804ce7442bff7a437aece542c))
* **icons:** append ?v=2 to the asset URLs to prompt Discord to refresh its file cache ([aaa0482](https://github.com/vyfor/cord.nvim/commit/aaa04820b0158cae0c72ad6553e7b3ac1ad391af))
* **icons:** append ?v=3 to the asset URLs to prompt Discord to refresh its file cache ([5d5e75a](https://github.com/vyfor/cord.nvim/commit/5d5e75a6bf141dc9f3a2963b6c47ddf3be71517b))
* **icons:** append ?v=4 to the asset URLs to prompt Discord to refresh its file cache ([fb4bcc4](https://github.com/vyfor/cord.nvim/commit/fb4bcc4d51751211c2ef9aed3611dad18ad884fb))
* **icons:** append ?v=5 to the asset URLs to prompt Discord to refresh its file cache ([ffdc72c](https://github.com/vyfor/cord.nvim/commit/ffdc72c2391b261602373ddfa5997ed17918356f))
* implement status codes for better error handling ([4bb5786](https://github.com/vyfor/cord.nvim/commit/4bb57863a14ff2547efbb083093d021b7e9a97c1))
* initial presence update is delayed by timer interval ([3f03ba4](https://github.com/vyfor/cord.nvim/commit/3f03ba464c7289bacd82f8d7e6569f67ae65ef74))
* invalid data size ([4b3bcb1](https://github.com/vyfor/cord.nvim/commit/4b3bcb1e8efe363d8e5bd2f8d5ec1554756ccd3a))
* issues with displaying the idle status ([0edce6f](https://github.com/vyfor/cord.nvim/commit/0edce6fbae407a480fa806119e9cc675ad45f71b))
* issues with refactoring ([efd0f50](https://github.com/vyfor/cord.nvim/commit/efd0f50c1b04b5e441299ff5bc9d9256a23e9cd8))
* language specific files not being recognized ([495b303](https://github.com/vyfor/cord.nvim/commit/495b303bb286271d389dee110b236b6e510375ad))
* log_level failing when set to nil ([5a8c5dd](https://github.com/vyfor/cord.nvim/commit/5a8c5dd1c85a4cf03105381a7f3a3ea23ba00170))
* logging ([cb0bc73](https://github.com/vyfor/cord.nvim/commit/cb0bc73a5d05ea7c0655c63139314baaa31db219))
* make `config.timer.reset_on_idle` reset idle status on both entry and exit ([5df8ca2](https://github.com/vyfor/cord.nvim/commit/5df8ca27b68167360fb90b3f11fa62101ae2c687))
* make the `Config` struct public ([8c6f789](https://github.com/vyfor/cord.nvim/commit/8c6f78931d3ecd5552a2f7575ddc295cb5564666))
* pad the tooltip with spaces in case it's too short ([2ca8849](https://github.com/vyfor/cord.nvim/commit/2ca88497da007e52fe2e9f42be2d71053009ba86))
* prevent sending identical activity updates ([62ffb46](https://github.com/vyfor/cord.nvim/commit/62ffb4654a4d2360d6b94d2be221c9766ea8857c))
* replace `os.clock()` with `vim.loop.hrtime()` due to inconsistent behaviour between different platforms ([061b166](https://github.com/vyfor/cord.nvim/commit/061b166a2a70d33279a1032e8131596d1736e9fd))
* repository button being displayed regardless of the configuration ([700e59e](https://github.com/vyfor/cord.nvim/commit/700e59e921088b64896a33a06b29d37c194055cc))
* send valid payload to clear presence ([5b49d82](https://github.com/vyfor/cord.nvim/commit/5b49d829a473d828b3896e54109229370d3e8136))
* sending identical updates ([76699c5](https://github.com/vyfor/cord.nvim/commit/76699c5f3029b878e43aa675d3fa6afe3906023a))
* **serialization:** escape special characters in the json ([c693475](https://github.com/vyfor/cord.nvim/commit/c6934755070ac1259fde0ae566097ea245083d85))
* small image not being overridden by the config ([b9b4f13](https://github.com/vyfor/cord.nvim/commit/b9b4f13a40082242984cb4918788f42e8ef1e1ad))
* swap the image tooltips, if swap_icons is set to true ([b4a1099](https://github.com/vyfor/cord.nvim/commit/b4a10994e9ecd52859650063d3136f768ea9f371))
* take the asset type into account when no matches are found ([4bbfb5c](https://github.com/vyfor/cord.nvim/commit/4bbfb5c517ef3b80e25a3e911b30e327572edc01))
* unix build ([3458ca1](https://github.com/vyfor/cord.nvim/commit/3458ca1edc0f4133b0460d989a63beff681eff80))
* update Git repository on `DirChanged` event ([b05bb4e](https://github.com/vyfor/cord.nvim/commit/b05bb4ef270ca126427078b28031bc41a5477bcd))
* update workspace when revisiting a previously opened buffer ([cf8cf2b](https://github.com/vyfor/cord.nvim/commit/cf8cf2b83655bbbc58f330fe58426bbc46ef72ba))
* use structs for functions with long arguments list due to ABI incompatibility issues on Apple Silicon ([d8e2387](https://github.com/vyfor/cord.nvim/commit/d8e2387bb15b59cb1f2bfe893a7ce95e8b0521aa))
* user commands ([0e5ca46](https://github.com/vyfor/cord.nvim/commit/0e5ca46d4c58041f0f8471ec1e433b4ca3c04d6d))
* workspace_blacklist ([c94361f](https://github.com/vyfor/cord.nvim/commit/c94361f389c06adfe8b0d0d10b4d0210df2e08fa))
* workspace_blacklist misbehaving (note: config.timer.enable is no longer a valid option) ([f2c9d99](https://github.com/vyfor/cord.nvim/commit/f2c9d9953b6d92ff702610bb8bd11bf05fdafec8))


### Features

* add .direnv directory to gitignore ([0c5cc06](https://github.com/vyfor/cord.nvim/commit/0c5cc06c4c54740cc3a730657b6ede9e4364b16f))
* add `astronvim` editor client ([963cdb6](https://github.com/vyfor/cord.nvim/commit/963cdb66afc62cbb1d54b7d3a9d2e5d968648396))
* add `astronvim` editor client ([2269dfc](https://github.com/vyfor/cord.nvim/commit/2269dfc3d389e24e304dde703d0a6bd429729479))
* add `config.log_level` ([191e613](https://github.com/vyfor/cord.nvim/commit/191e6130e3ba93fdc11d6e54f4c6c97a0190a9d6))
* add a configuration option to replace the default idle icon ([3ebf630](https://github.com/vyfor/cord.nvim/commit/3ebf63036bfba8944cdf1298c244770aacdd2da2))
* Add a user command for changing the workspace name ([fa1a47b](https://github.com/vyfor/cord.nvim/commit/fa1a47b46c9f046d3bc15b26bcb958099bfdd2e5))
* add a user command for changing workspace ([f8c289c](https://github.com/vyfor/cord.nvim/commit/f8c289ce963b1317dcd1eaee77f969b6b7dbc969))
* add an option to configure the logging level ([273d454](https://github.com/vyfor/cord.nvim/commit/273d4546ac2bdc610f35be531bbf1aa4b360dfd3))
* add custom buttons ([9f00000](https://github.com/vyfor/cord.nvim/commit/9f0000074d75005516e936f8d09305c46e908348))
* add fern as file browser ([3ac290c](https://github.com/vyfor/cord.nvim/commit/3ac290c2ab330df2070042985549acbee995ecd4))
* add file icon for ahk ([4246e0f](https://github.com/vyfor/cord.nvim/commit/4246e0fa89851827dbd83c6242cb03302f624324))
* add file icon for clojure ([bd6c32b](https://github.com/vyfor/cord.nvim/commit/bd6c32b8eed5e3740bec72ca44d3e73d774951eb))
* add file icon for crystal ([7eb6a73](https://github.com/vyfor/cord.nvim/commit/7eb6a73b1f79c44c8534c3da282db60531246e90))
* add file icon for d lang ([d2a66d4](https://github.com/vyfor/cord.nvim/commit/d2a66d43afe66ccd61c7095ec0e7aed806e2bb7c))
* add file icon for docker ([de2c389](https://github.com/vyfor/cord.nvim/commit/de2c389dd57acc03056bf8c3be7bcb15884fe97b))
* add file icon for elixir ([b6324f0](https://github.com/vyfor/cord.nvim/commit/b6324f0e169b12a6000ed1d1502de3869d177464))
* add file icon for erlang ([ff9627d](https://github.com/vyfor/cord.nvim/commit/ff9627d838a63a8d50947cffe669c3321e471f93))
* add file icon for F# ([b600aa9](https://github.com/vyfor/cord.nvim/commit/b600aa9f18ed3d97191d5958d34f972d7ca75533))
* add file icon for git ([041bff7](https://github.com/vyfor/cord.nvim/commit/041bff759a446e1bd9fe416d3443ddf4abc8610b))
* add file icon for GML ([98a94bb](https://github.com/vyfor/cord.nvim/commit/98a94bbb605875b1ded90b654fdc06ade0dd9060))
* add file icon for gradle ([f937c28](https://github.com/vyfor/cord.nvim/commit/f937c28fbaa3661e4d92831dc3372ab633c64849))
* add file icon for groovy ([e732a44](https://github.com/vyfor/cord.nvim/commit/e732a44d0a39042d73a29996990c6c0fb1afd0c4))
* add file icon for latex ([737b765](https://github.com/vyfor/cord.nvim/commit/737b7659feef481f31e6d37c3e38a96bbdedbbeb))
* add file icon for license files ([e2227c2](https://github.com/vyfor/cord.nvim/commit/e2227c22411cf1cc95e9981b8802ebe207470787))
* add file icon for lisp ([71c9511](https://github.com/vyfor/cord.nvim/commit/71c951139e13a74f3ccc410c5d44f04700597b24))
* add file icon for nim ([099dd72](https://github.com/vyfor/cord.nvim/commit/099dd7289a0f21dfc8284ad128b5b303fdf24630))
* add file icon for nix ([2626769](https://github.com/vyfor/cord.nvim/commit/26267697ac3a412a71d0ad85b9f01b24e96eb602))
* add file icon for ocaml ([9cac03c](https://github.com/vyfor/cord.nvim/commit/9cac03ca7c88674e873fc229b56a64e7abdc4a77))
* add file icon for pascal ([e06c339](https://github.com/vyfor/cord.nvim/commit/e06c339e1d0f9cfb84454be064e95990eed7960c))
* add file icon for postcss ([7b489dc](https://github.com/vyfor/cord.nvim/commit/7b489dcc70da4ee4dfb675613a0f689fd3f93277))
* add file icon for r lang ([dbfbdcb](https://github.com/vyfor/cord.nvim/commit/dbfbdcb6b5fa08cce8dca5cc7973c5243a116ba5))
* add file icon for sass ([568b190](https://github.com/vyfor/cord.nvim/commit/568b190c6dc750e1abce41f175c9513f526634cb))
* add file icon for svelte ([fa857d3](https://github.com/vyfor/cord.nvim/commit/fa857d36f23a7df38b907047b2da99c2315ab166))
* add file icon for v lang ([7e1bd31](https://github.com/vyfor/cord.nvim/commit/7e1bd3104b2e99b2e8217c5d8740bfa12279efd2))
* add file icon for vue ([2be12a8](https://github.com/vyfor/cord.nvim/commit/2be12a83b3ee5959146c53e832ecf56da8aa134e))
* add file icon for zig ([c74a64d](https://github.com/vyfor/cord.nvim/commit/c74a64d280b6cee44e142134d275cdaa82bb2a25))
* Add icon for Julia language ([a678779](https://github.com/vyfor/cord.nvim/commit/a678779425864f26ee6c8825a0e69dd9fd767912))
* Add icon for Maven ([91152a8](https://github.com/vyfor/cord.nvim/commit/91152a889ae7d82facdfa30d25d401426c842a11))
* add logging ([8d9957c](https://github.com/vyfor/cord.nvim/commit/8d9957cfd967a97d83d6c88cccbf2e59bfee188d))
* add nix flake + direnv ([b62e959](https://github.com/vyfor/cord.nvim/commit/b62e95982644af9c6149dc58889d86a3497370b1))
* add phoenix icon + eelixir filetype ([a3d65aa](https://github.com/vyfor/cord.nvim/commit/a3d65aa444227e65629d451de4b5fd36d132d08d))
* Add support for custom icons ([6ac0239](https://github.com/vyfor/cord.nvim/commit/6ac0239330ffd4490d37a0582827ac53c20253d9))
* add support for Flatpak and Snap IPC paths ([b431eb2](https://github.com/vyfor/cord.nvim/commit/b431eb284d43cb5c7a2ad1a5f47142b410b127c6))
* add support for Flatpak and Snap IPC paths ([57d1111](https://github.com/vyfor/cord.nvim/commit/57d1111bf56f50bba0367d09a70d6dbcf2eca891))
* add support for git related plugins ([a603a95](https://github.com/vyfor/cord.nvim/commit/a603a95b66ae42d29d385230348ce3db8f095ae7))
* add support for lsp managers ([8bca29e](https://github.com/vyfor/cord.nvim/commit/8bca29eb1497efa6b986fa763869881f7a7169b3))
* Add support for LSP managers ([c9c8593](https://github.com/vyfor/cord.nvim/commit/c9c859379bf2bf070f52c804268a3ab3cea326a3))
* add support for more file icons ([8529e6f](https://github.com/vyfor/cord.nvim/commit/8529e6fc03c2bcfb6d41832d9837cc90317af92d))
* add support for plaintex filetype ([2a7bd9d](https://github.com/vyfor/cord.nvim/commit/2a7bd9dd97e0f8c363162f6c4943773cb80890b2))
* add user commands for managing the idle status ([06c4ae8](https://github.com/vyfor/cord.nvim/commit/06c4ae8e177305a8d58f6873027744c17131fe86))
* add workspace blacklist ([b5fd3ee](https://github.com/vyfor/cord.nvim/commit/b5fd3eede26cd3b234a0c919ce9f2285939b7a15))
* add yazi as a file explorer ([fd5e423](https://github.com/vyfor/cord.nvim/commit/fd5e4236c85de62e844593b799ddc095e92fcc00))
* allow string values for asset types ([9ee4094](https://github.com/vyfor/cord.nvim/commit/9ee4094ed90c1ae3c1ffb0ece0999a781bcedd03))
* allow swapping the position of fields ([1dc822b](https://github.com/vyfor/cord.nvim/commit/1dc822b30722274c651bd3b413c20601e54acf2d))
* allow swapping the positions of the editor and language icons ([afad2bb](https://github.com/vyfor/cord.nvim/commit/afad2bb7c66e9564f7faf669f1f984cb01712e4f))
* darken backgrounds of the icons ([aac34bd](https://github.com/vyfor/cord.nvim/commit/aac34bd84db07b73467872e63ec8eb8b23f2d01d))
* darken backgrounds of the icons ([8cc1b6a](https://github.com/vyfor/cord.nvim/commit/8cc1b6afacf8f77326e0e655995f165de350537a))
* hide cursor position text when at position 1:1 ([6444711](https://github.com/vyfor/cord.nvim/commit/64447113f348a5b5dd8b5e4b8449ae04b728a70c))
* hide workspace text on idle ([af23462](https://github.com/vyfor/cord.nvim/commit/af23462a4734e9873dc961706251a386ab06ed07))
* **icons:** add a default icon for lsp managers ([bcd37f0](https://github.com/vyfor/cord.nvim/commit/bcd37f0e7495c4d6ca7a19ccbf5c92fa689d0ab2))
* **icons:** add a file icon for CUDA ([7ce67bf](https://github.com/vyfor/cord.nvim/commit/7ce67bf5ccda6838a33a12d8b34fd9b25077dd5e))
* **icons:** add a file icon for Haxe ([0e1e7b4](https://github.com/vyfor/cord.nvim/commit/0e1e7b4d25c8751c65a3b9f61857e01d0f9abe12))
* **icons:** add icon for julia language ([58dd29d](https://github.com/vyfor/cord.nvim/commit/58dd29d35a2c331b1e8067e5607da1cce60541ec))
* **icons:** add icon for maven ([ebca692](https://github.com/vyfor/cord.nvim/commit/ebca6925f770e1fd245d35256cb61090b3a677f9))
* **icons:** add icon for quarto ([d514aec](https://github.com/vyfor/cord.nvim/commit/d514aec22e0b43ca81207402a8bbe0ff4c8ad81f))
* **icons:** add icon for zsh ([c04e8fd](https://github.com/vyfor/cord.nvim/commit/c04e8fda9ed0f97bbcde760ce88c2a6fddb2abe4))
* **icons:** provide an icon for Astro ([fd8016f](https://github.com/vyfor/cord.nvim/commit/fd8016fb952776efe778f0149c1f1445ef1877f6))
* **icons:** update java icon ([a9b975d](https://github.com/vyfor/cord.nvim/commit/a9b975de376df4c6b97afbb25ca5e33e987e0850))
* **icons:** update kotlin icon ([6130cdb](https://github.com/vyfor/cord.nvim/commit/6130cdbe5a507a6ecd57d6ac351edf68c84e809f))
* **icons:** update lua icon ([004a79c](https://github.com/vyfor/cord.nvim/commit/004a79c622c9c874a16a393f624098c305dd9519))
* **icons:** update shell icon ([dde71a8](https://github.com/vyfor/cord.nvim/commit/dde71a8161fe726ca0245b67de85489081cf7d23))
* **icons:** update shell icon ([1f3e08c](https://github.com/vyfor/cord.nvim/commit/1f3e08cbfc4593f956edf98c2cabe0ae5e217758))
* implement `config.assets['Cord.override']` ([#80](https://github.com/vyfor/cord.nvim/issues/80)) ([0777f91](https://github.com/vyfor/cord.nvim/commit/0777f9132d6a21b2ffed046082653131429af2cc))
* implement `swap_icons` ([758e0dc](https://github.com/vyfor/cord.nvim/commit/758e0dc4f460f6eb095013002cb39d38f23c2554))
* improve error handling ([70d6bb4](https://github.com/vyfor/cord.nvim/commit/70d6bb457a6846f02a936d8e0fce4d7ae1cc35d6))
* increase connection timeout to 60 seconds ([bf121e2](https://github.com/vyfor/cord.nvim/commit/bf121e214faecd173cc897692f11bec27cc8dd03))
* **lua:** add support for BSD ([c3ee1aa](https://github.com/vyfor/cord.nvim/commit/c3ee1aaa210b4e9c433afe73b4b075b9fdffa47c))
* properly close pipe connection ([8dde573](https://github.com/vyfor/cord.nvim/commit/8dde573d6a85e64fe9d142351a99f7fc91c4dee7))
* provide a way to override the default file icon, which is used when filetype was not detected ([07e889a](https://github.com/vyfor/cord.nvim/commit/07e889ad7da5ff52f3e2fc4875ab2f9232ac044c))
* provide an option to override all icons with a custom one ([e679870](https://github.com/vyfor/cord.nvim/commit/e6798702770c5a8dcaf4d12a1428598eec2e0eac))
* shut down connection in case it wasn't established within 15 seconds ([c7f0f7a](https://github.com/vyfor/cord.nvim/commit/c7f0f7a86038f41bd97c30d63fcfeb8152dcb774))
* support custom icons ([9627657](https://github.com/vyfor/cord.nvim/commit/9627657700de3fc7a57855e5f9e9c1bda0b12813))
* support defining file assets with a string ([824455d](https://github.com/vyfor/cord.nvim/commit/824455d56a2273dfe47823edcdbcff8a0094bb15))
* support executables launched via toggleterm ([4119ef4](https://github.com/vyfor/cord.nvim/commit/4119ef4a15c03205fe23f9da5618c0d10c0559f7))
* support multiple git protocols ([2af4b85](https://github.com/vyfor/cord.nvim/commit/2af4b8556b090491f270041440fc69f695112971))
* Support multiple Git protocols ([190e836](https://github.com/vyfor/cord.nvim/commit/190e8362a5ea0a0161227a0e5aa45f97eb84e039))
* support the rest of the git related filetypes ([20c390e](https://github.com/vyfor/cord.nvim/commit/20c390e9e4bbc3dc1640c6e42b23c04ef4189ece))
* use current file's directory as the workspace instead of cwd ([7ef94fc](https://github.com/vyfor/cord.nvim/commit/7ef94fc3df78a3374eb6e9090b0d61687e882f21))


### Performance Improvements

* move crucial components to the Rust codebase ([e6e1cf3](https://github.com/vyfor/cord.nvim/commit/e6e1cf32cb208a28bcc376b6ccd0a4bed9eee9f9))
* move crucial components to the Rust codebase ([3cb21e5](https://github.com/vyfor/cord.nvim/commit/3cb21e5eb5d33313a52ae5d3ed3eee0078022c74))
* optimize `find_git_repository` ([45bc551](https://github.com/vyfor/cord.nvim/commit/45bc5515457950627b5942737e425334d3076551))
* remove unnecessary vector allocations ([160a9d1](https://github.com/vyfor/cord.nvim/commit/160a9d177174f22917e565b9d8d25ed7ccbcb044))
* set `assets` to `nil` by default ([586ceed](https://github.com/vyfor/cord.nvim/commit/586ceedb2ef25edf85d520c38aa2510e98bcc11f))