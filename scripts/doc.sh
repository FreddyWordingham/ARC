cd $ARC_DIR;

cargo doc;
cp -r $ARC_DIR/target/doc $ARC_DIR/doc;
echo "<meta http-equiv=refresh content=0;url=arc/index.html>" > $ARC_DIR/doc/index.html;

cd -;
