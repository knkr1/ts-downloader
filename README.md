# .ts Downloader

A basic .ts downloader that able to download .ts files from a link provided.

I have been using this for installing EURO2024 FINAL DRAW, which got removed for the unknown incident happened on live.

## Requirements

The only requirement is that you should have ffmpeg command line tool installed and accessible from the command line.
Also it's a rust program. You should have rust installed on your computer to build it. Otherwise, release should be available.

## Setup

> cargo build

## Tutorial

What program do is just installing the parts of videos, which are .ts and convert them into .mp4 file.
You must get the link by yourself, which should look like this:

`https://live-uefa-com.akamaized.net/hls/live/2036934/021223_EURO24FINAL/master_1_02500.ts?l2v=1`

What you see above is a example that I got from the offical EURO2024 draw. By opening the network tab at the developer tools, you can see the same link comes over to install .ts file.
And if you check 3 different links:

`https://live-uefa-com.akamaized.net/hls/live/2036934/021223_EURO24FINAL/master_1_02500.ts?l2v=1`
`https://live-uefa-com.akamaized.net/hls/live/2036934/021223_EURO24FINAL/master_1_02501.ts?l2v=1`
`https://live-uefa-com.akamaized.net/hls/live/2036934/021223_EURO24FINAL/master_1_02502.ts?l2v=1`

It seems like a specific part being used to numberize .ts files.


When we open the program, it will ask you to create a new setting, will ask for the link and start-end range.

The link should be like this: `https://live-uefa-com.akamaized.net/hls/live/2036934/021223_EURO24FINAL/master_1_0{}.ts?l2v=1`
I leaved one 0 alone because it gives error when I try 2500, but works when I do 02500.

I came to the beginning of the video, and saw the live stream starts from 1500. file. So I wrote 1500 as the start.
And 3200 were the end, so our end.

After we done the setting, a file in our directory named `.ts_downloader_settings.txt` which saves our settings as `link,start,end`
We can change our setting later, or it will ask for a new setting if we delete it.

We may click opinion 1 for install the files on the range, and they will be get saved in ts_files folder.
Using 2. opinion will convert all of the .ts files into a mp4 /w ffmpeg.

And we can get our video as output.mp4



# WARNING

This repository and the tutorial is created for educational purposes. I'm not teaching or guiding people about doing illegal activity.
