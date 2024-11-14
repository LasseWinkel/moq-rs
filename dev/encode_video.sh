#!/bin/bash

# Input and output variables
INPUT_MP4="bbb.mp4"
YUV_FILE="bbb.yuv"
RESOLUTION="1280x720"    # Adjust this based on your video resolution
PIX_FMT="yuv420p"         # Pixel format for YUV file
FRAME_RATE=24             # Frame rate in FPS
NUM_FRAMES_TO_DECODE=1080 # Number of frames to decode
BITRATE="6000k"           # Video bitrate
OUTPUT_FILE="frankenstein.mp4"
OUTPUT_YUV_FILE="frankenstein.yuv"

# Step 1: Decode MP4 to YUV
ffmpeg -i $INPUT_MP4 -s:v $RESOLUTION -pix_fmt $PIX_FMT -r $FRAME_RATE -frames:v $NUM_FRAMES_TO_DECODE -f rawvideo $YUV_FILE

# Step 2: Encode the first 840 frames with GOP size of 48 (15 seconds setup + 20 seconds good network)
ffmpeg -s:v $RESOLUTION -pix_fmt $PIX_FMT -f rawvideo -r $FRAME_RATE -i $YUV_FILE \
    -frames:v 600 -g 48 -c:v libx264 -profile:v high -preset medium -b:v $BITRATE \
    -x264-params "keyint=48:min-keyint=48" -an segment1.mp4

# Step 3: Encode the next 480 frames with GOP size of 12 (20 seconds bad network)
ffmpeg -s:v $RESOLUTION -pix_fmt $PIX_FMT -f rawvideo -r $FRAME_RATE -i $YUV_FILE \
    -ss 25 -frames:v 240 -g 12 -c:v libx264 -profile:v high -preset medium -b:v $BITRATE \
    -x264-params "keyint=12:min-keyint=12" -an segment2.mp4

# Step 4: Encode the next 720 frames with GOP size of 48 (20 seconds good network, 10 seconds extra buffer to end stream)
ffmpeg -s:v $RESOLUTION -pix_fmt $PIX_FMT -f rawvideo -r $FRAME_RATE -i $YUV_FILE \
    -ss 35 -frames:v 240 -g 48 -c:v libx264 -profile:v high -preset medium -b:v $BITRATE \
    -x264-params "keyint=48:min-keyint=48" -an segment3.mp4

# # Step 5: Encode the bbb.yuv with GOP size of 48
# ffmpeg -s:v $RESOLUTION -pix_fmt $PIX_FMT -f rawvideo -r $FRAME_RATE -i $YUV_FILE \
#     -g 48 -c:v libx264 -profile:v high -preset medium -b:v $BITRATE \
#     -x264-params "keyint=48:min-keyint=48" -an bbb-48.mp4

# # Step 6: Encode the bbb.yuv with GOP size of 24
# ffmpeg -s:v $RESOLUTION -pix_fmt $PIX_FMT -f rawvideo -r $FRAME_RATE -i $YUV_FILE \
#     -g 24 -c:v libx264 -profile:v high -preset medium -b:v $BITRATE \
#     -x264-params "keyint=24:min-keyint=24" -an bbb-24.mp4

# # Step 7: Encode the bbb.yuv with GOP size of 12
# ffmpeg -s:v $RESOLUTION -pix_fmt $PIX_FMT -f rawvideo -r $FRAME_RATE -i $YUV_FILE \
#     -g 12 -c:v libx264 -profile:v high -preset medium -b:v $BITRATE \
#     -x264-params "keyint=12:min-keyint=12" -an bbb-12.mp4

# Create a file list for concatenation
echo "file 'segment1.mp4'" > filelist.txt
echo "file 'segment2.mp4'" >> filelist.txt
echo "file 'segment3.mp4'" >> filelist.txt

# Step 8: Concatenate all segments into the final output
ffmpeg -f concat -safe 0 -i filelist.txt -c copy $OUTPUT_FILE

# Step 9: Decode frankenstein MP4 to YUV
ffmpeg -i $OUTPUT_FILE -s:v $RESOLUTION -pix_fmt $PIX_FMT -r $FRAME_RATE -f rawvideo $OUTPUT_YUV_FILE

# Clean up intermediate files
rm segment1.mp4 segment2.mp4 segment3.mp4 filelist.txt $YUV_FILE

echo "Encoding complete! Output saved as $OUTPUT_FILE"
