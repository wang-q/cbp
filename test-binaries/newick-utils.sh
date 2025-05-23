#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test Newick tree file
echo "-> Creating test Newick file"
cat > "HRV.nw" << 'EOF'
(((((((((HRV85_1:0.114608,(HRV89_1:0.219212,HRV1B_1:0.123339):0.076821):0.043577,(HRV9_1:0.258951,(HRV94_1:0.000000,HRV64_1:0.064173):0.000000):0.131621):0.020743,(HRV78_1:0.166685,HRV12_1:0.024545):0.227116):0.074814,(HRV16_1:0.204300,HRV2_1:0.529712):0.224056):0.105454,HRV39_1:0.044427):0.656750,((HRV14_1:0.080836,(HRV37_1:0.225838,HRV3_1:0.090367):0.080898):0.201351,(HRV93_1:0.195377,HRV27_1:0.000000):0.081157):0.632018):0.317738,(HEV68_1:0.036279,(HEV70_1:0.264011,(((((POLIO1A_1:0.173760,POLIO2_1:0.087100):0.168238,POLIO3_1:0.163550):0.068253,(COXA17_1:0.152096,COXA18_1:0.155755):0.098067):0.878785,COXA1_1:0.161008):0.345592,((COXB2_1:0.562379,ECHO6_1:0.270981):0.240589,ECHO1_1:0.004346):0.936634):0.770246):0.051896):0.438878):1.235120,COXA14_1:0.121281):0.544944,COXA6_1:0.675458,COXA2_1:0.557975);
EOF

# Test nw_stats
echo "-> Testing nw_stats"
STATS_OUTPUT=$($(cbp prefix bin)/nw_stats "HRV.nw")

assert 'echo "${STATS_OUTPUT}" | grep -q "leaves:	30"' "Expected 'leaves: 30' in stats output"
