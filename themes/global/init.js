$(function() {
  $("p.markdown").each(function(e) {
    var txt = $(this).text();
    $(this).html(marked(txt));
  });
});
