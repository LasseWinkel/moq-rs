ffmpeg -hide_banner -i bbb.mp4 -vf "select='between(n\,0\,600)',setpts=PTS-STARTPTS" -g 48 -keyint_min 48 -an -r 24 bbb-merge-trimmed-0-600-gop-48.mp4
ffmpeg -hide_banner -i bbb.mp4 -vf "select='between(n\,601\,840)',setpts=PTS-STARTPTS" -g 12 -keyint_min 12 -an -r 24 bbb-merge-trimmed-601-840-gop-12.mp4
ffmpeg -hide_banner -i bbb.mp4 -vf "select='between(n\,841\,1440)',setpts=PTS-STARTPTS" -g 48 -keyint_min 48 -an -r 24 bbb-merge-trimmed-841-1440-gop-48.mp4

