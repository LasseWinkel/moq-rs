# VMAF (check vmaf installation)
ffmpeg -filters | grep vmaf

ffmpeg -i bbb.mp4 -c:v libx264 -g 12 -keyint_min 12 -sc_threshold 0 -bf 0 -c:a copy bbb-12.mp4
ffmpeg -i bbb.mp4 -c:v libx264 -g 24 -keyint_min 24 -sc_threshold 0 -bf 0 -c:a copy bbb-24.mp4
ffmpeg -i bbb.mp4 -c:v libx264 -g 48 -keyint_min 48 -sc_threshold 0 -bf 0 -c:a copy bbb-48.mp4

ffmpeg -i bbb-12.mp4 -i bbb.mp4 -lavfi "libvmaf=log_fmt=json:log_path=output-12.json:n_threads=6" -f null -
ffmpeg -i bbb-24.mp4 -i bbb.mp4 -lavfi "libvmaf=log_fmt=json:log_path=output-24.json:n_threads=6" -f null -
ffmpeg -i bbb-48.mp4 -i bbb.mp4 -lavfi "libvmaf=log_fmt=json:log_path=output-48.json:n_threads=6" -f null -

# dump video and audio
ffprobe -show_frames -print_format json bbb.mp4 > frames.json

# dump only video
ffprobe -select_streams v:0 -show_frames -print_format json bbb.mp4 > frames.json
ffprobe -select_streams v:0 -show_frames -print_format json bbb-12.mp4 > frames-12.json
ffprobe -select_streams v:0 -show_frames -print_format json bbb-24.mp4 > frames-24.json
ffprobe -select_streams v:0 -show_frames -print_format json bbb-48.mp4 > frames-48.json
