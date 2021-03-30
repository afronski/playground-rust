#!/usr/bin/env bash

MODE=${1:-slow}

python3 generate-credentials-file.py

export GST_PLUGIN_PATH="/home/afronski/clones/amazon-kinesis-video-streams-producer-sdk-cpp/build"

case ${MODE} in

    slow)
        gst-launch-1.0 v4l2src do-timestamp=TRUE device=/dev/video0 ! videoconvert ! video/x-raw,format=I420,width=640,height=480,framerate=30/1 ! videorate drop-only=true ! video/x-raw,framerate=1/10 ! x264enc bframes=0 key-int-max=1 bitrate=500 tune=zerolatency ! video/x-h264,stream-format=avc,alignment=au,profile=baseline ! kvssink stream-name="webcam-test" storage-size=512 credential-path="./credentials.private" aws-region="eu-west-1"
        ;;

    fast)
        gst-launch-1.0 v4l2src do-timestamp=TRUE device=/dev/video0 ! videoconvert ! video/x-raw,format=I420,width=640,height=480,framerate=30/1 ! x264enc bframes=0 key-int-max=45 bitrate=500 ! video/x-h264,stream-format=avc,alignment=au,profile=baseline ! kvssink stream-name="webcam-test" storage-size=512 credential-path="./credentials.private" aws-region="eu-west-1"
        ;;

    *)
        echo "${0} [fast|slow]"
        ;;

esac
