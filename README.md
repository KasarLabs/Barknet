# Barknet: Bitcoin Sovereign Rollup

<div align="center">
    <img src="https://kasar.io/_next/image?url=%2F_next%2Fstatic%2Fmedia%2FkasarLogo.0513044c.png&w=640&q=75" height="128">
</div>

A sovereign rollup based on the **Madara** engine and powered by **Starknet**. Proudly developed by Kasar Labs in collaboration with Taproot Wizards.

## Quick Links

- [Barknet](https://github.com/KasarLabs/barknet)
- [Bitcoin-da](https://github.com/KasarLabs/bitcoin-da)
- [Madara](https://github.com/keep-starknet-strange/madara)

## Getting Started

For detailed guidelines on setting up and utilizing Barknet, please refer to the official documentation provided in the repository.

## Contribute

- [Madara Explorer](https://github.com/lambdaclass/madara_explorer) by the great
  [LambdaClass](https://lambdaclass.com/) team 🫶: A block explorer for Madara.
- [Madara Infra](https://github.com/keep-starknet-strange/madara-infra): A
  collection of scripts and tools to deploy and manage Madara on different
  environments (e.g. AWS, docker, ansible, etc.). It also contains the
  [Starknet Stack](https://github.com/keep-starknet-strange/madara-infra/blob/main/starknet-stack/docker-compose.yml)
  demo `docker-compose` file.
- [Madara Kit Application](https://github.com/keep-starknet-strange/madara-app):
  A simple application that demonstrates how to use Madara. Deployed on
  `https://app.madara.zone`.
- [Madara Docsite](https://github.com/keep-starknet-strange/madara-app): The
  source code of the Madara documentation website. Deployed on
  `https://docs.madara.zone`.
- [Madara Tsukuyomi](https://github.com/keep-starknet-strange/madara-tsukuyomi):
  The source code of the Madara Desktop App. A friendly GUI to start a Madara
  node and interact with it.
- [App Chain Template](<(https://github.com/keep-starknet-strange/madara-app-chain-template)>):
  A ready to use template that allows you to easily start an app chain.

## 📣 Building App Chains

> Do NOT fork this repo and build your app chain on top unless completely
> necessary. By adding changes using forking, you will have to periodically
> rebase (and solve conflicts) to remain updated with the latest version of
> Madara.

One of the main features of Madara is to allow users to start their app chains
that support Cairo contracts and Starknet like blocks. Hence, to make it easy
for users to build a custom app chain, we have created an
[app-chain-template](https://github.com/keep-starknet-strange/madara-app-chain-template)
which imports Madara as a pallet. This removes all the boilerplate code and
allows you to focus on code only relevant to your app chain. Moreover, updating
Madara is as simple as updating the pallet version.

## License

Barknet is licensed under the **Apache 2.0 license**.

---

Let's scale Bitcoin togethers! 🚀