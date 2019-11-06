function draw() {
  // すべての画像に対するループ処理
  for (var i = 0; i < document.images.length; i++) {
    // 額縁の画像用の canvas は追加しない
    if (document.images[i].getAttribute("id") != "frame") {
      // canvas 要素を作成
      canvas = document.createElement("canvas");
      canvas.setAttribute("width", 132);
      canvas.setAttribute("height", 150);

      // 画像の前に挿入
      document.images[i].parentNode.insertBefore(canvas, document.images[i]);

      ctx = canvas.getContext("2d");

      // canvas に画像を描く
      ctx.drawImage(document.images[i], 15, 20);

      // 額縁を追加
      ctx.drawImage(document.getElementById("frame"), 0, 0);
    }
  }
}
