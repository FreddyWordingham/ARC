#!/bin/bash


for y in `seq 900 50 1500`
    do
        touch output/raman/Ramans.txt
        python scripts/raman/adjust_scatt.py $y
        for x in `seq -0.01 0.001 0.01`
            do
                for value in `seq 1 1 10`
                    do
                        python scripts/raman/slab_move.py $x
                        cargo run --bin raman --release parameters.json
                    done
            done
        mv "output/raman/Ramans.txt" "output/raman/param_test/Ramans_$y.txt"
    done
