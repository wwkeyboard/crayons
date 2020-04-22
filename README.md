# crayons

HTML generation for people who don't want to do HTML.

## Why does this exist?
Let's face it, the web is here to stay. This means at some point
you'll probably need to use a browser to display some information and
that means HTML. There are lots of great HTML templating crates, but
they require you have a good handle on HTML to use them. The goal of
Crayons is to provide some lines for you to color inside.

The goal is to lean on Rust's type system and on RLS to guide you
through building a simple HTML document, if it compiles it should be
good enough. You create the root document, and the functions available
on that document represent the html elements that can be contained in
that document. Those functions' return types also have functions that
represent what can be contained. So if you have a quick familiarity
with HTML you can create a new document, hit `tab` or whatever your
code completion key is, and rls will give you a list of potential next
steps.

Crayon's target applications are both quick-and-dirty hackathon-ish
projects and proof of concepts as well as making admin panels, status
pages, documentation pointers, etc. less difficult and hopefully
better looking. Your SREs will love you if an HTTP request to the root
of your project includes a quick description of what it does and links
to the docs and run book.

## Performance
Crayons isn't meant for performance critical portions of your
application. It's performance goals are to not be embarrassingly
slow. Outside of that it's primary concerns are to be easy to use and
generate relatively correct HTML.
