#!/bin/sh

##Place PBS directives here
#PBS -N alexthesis 
#PBS -l nodes=1:ppn=36
#PBS -l walltime=72:00:00
#PBS -M ahansen2@trinity.edu
#PBS -m ae

##Place Linux commands to run on the remote node here
~/thesis/genetic_painting/target/release/genetic_painting --file 'image.jpg' --iterations 100 --population 1000 --selector parmaximize --strokes 500 --strokewidth 5 -v 1
