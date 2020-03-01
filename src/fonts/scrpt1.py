import os

with open("matches.txt","a") as f:
    f.write("match self{\n")
for font in os.listdir():
    with open("matches.txt","a") as ff:
        font1 = font.split('.')
        #print(font)
        if font1[1]=='ttf':
            font2 = font1[0]
            '''ff.write("\t\t\t")
            ff.write(font1[0].replace('-',''))
            ff.write(",\n")'''
            ff.write("\tFonts::")
            ff.write(font2.replace('-',''))
            ff.write("=>{ let ttf1 = include_bytes!( \"fonts/")
            ff.write(font1[0])
            ff.write(".ttf\");\n")
            ff.write("\treturn ttf1;},")
            ff.write("\n")
with open("matches.txt","a") as fff:
    fff.write("}")

