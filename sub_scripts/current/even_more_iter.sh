#!/bin/sh

##Place PBS directives here
#PBS -N alexthesis 
#PBS -l nodes=1:ppn=36
#PBS -l walltime=72:00:00
#PBS -M ahansen2@trinity.edu
#PBS -m ae

##Place Linux commands to run on the remote node here
time ~/thesis/genetic_painting/target/release/genetic_painting --file 'image.jpg' --iterations 700 --population 30 --selector parmaximize --strokes 5 --strokewidth 20 -v 1 -m 5 -M 30 -r --maxcurve 5
