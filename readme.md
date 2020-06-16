# Onrain befriender

This project is meant for the people of onrain who just want a lot of friends! 
Onrain is SchoolRP's official social media platform, it's located at [this link](https://social.schoolrp.net/).

## Installation

Just head over to [the releases page](https://github.com/zegevlier/onrain-befriender/releases) and download the latest version!

## Usage

Once you have downloaded the zip file, extract it somewhere. You will see two files, a config and a .exe file. Please see below on how to fill out the config. once you have done that, just double click the .exe to run! You might need to allow it through your firewall.

## Configuration

To use the config file, copy the `config.base.yaml` to `config.yaml` and set the values appropriately. Make sure you put your value between the `""`, if you don't the program might not be able to read it. There are multiple options in the config file, here you will find a list with what they all mean and where to find them.

- **Cookies -> user_id:**  This is the id you find in the address bar after the `/profile-` when you are on your own profile page. Alternatively, you can find it in your cookies, which I will explain how to see in the next bullet point.
- **Cookies -> user_hash:**  You need to get this value from your cookies, this might sound a bit intimidating but it's really easy if you know how. First of all, find a browser extension that lets you see (and edit) your cookies, I would recommend "[EditThisCookie](https://chrome.google.com/webstore/detail/editthiscookie/fngmhnnpilhplaeedifhccceomclgfbg)", it's free and available on Chrome and Opera. Once you have installed it, go to onrain and click the icon in the top right of your screen. Then, look for the `cored9e2user_hash name`, click it and copy the contents into this field!
- **last_known_good:**  This should be the number of the last known good account, this is so accounts that are deleted don't mess up the bot like they normally do. At the time of writing the highest known account id is ~1000, so that is the default value in this file.
- **start_number:**  This is the id of the first account you want the bot to try and befriend, this should be set to the same number as it was at the moment you closed the bot to avoid spamming the server with unnecessary requests and to avoid wasting your own time.

## F.A.Q.

 - **Is my account going to get banned if I use this?**
  I don't know! I know some admins knew about me using my bot and never complained, but I don't know what it will be like when a lot of people start using it. AFAIK they don't have a (good) way of knowing if anyone is using it, but don't blame me if you get banned.
 - **Can you add *insert feature here*?**
  No idea! I've just started learning Rust and I have no idea if I can do it and if it would even fit with the program, just make an issue and we'll see what happens. Just don't be offended if I don't implement it.
 - **This is great and it has given me so many friends, is there any way I can repay you? (;**
  Yes there is! I am sometimes online under the account name of zegevlier, you can send me yen if you want. And if you are, for some reason, feeling extra generous, I have a BTC address: `bc1q23ggmf2wxl84794uam0lmdhkgyhn5pzhltlwex`. Also, if you ever need someone to make a little program, hit me up on discord at zegevlier#2959!
 - **Help! My antivirus blocks this program from running!** 
  This can happen, there is no way around it. But luckily I released my source code so you can build it for yourself if you don't trust me! If you want you can look though the source code to see for yourself that this is not a virus and I'm not sending your login info to myself.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. If you would like to request a feature, also open an issue! There are currently plans to make it accept your normal login so users don't have to faff around with cookies, so that might come at a later date.
