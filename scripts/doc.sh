cd $ARC_DIR;

cargo doc;
rm -r $ARC_DIR/docs;
mv $ARC_DIR/target/doc $ARC_DIR/docs;
echo "<meta http-equiv=refresh content=0;url=arc/index.html>" > $ARC_DIR/docs/index.html;

cd -;
