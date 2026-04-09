import re

path = r'c:\Users\minec\Documents\GitHub\code\apps\app-frontend\src\components\ui\SplashScreen.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()

# Replace SVG block
svg_pattern = re.compile(r'<svg.*?</svg>', re.DOTALL)
new_content = svg_pattern.sub('<img :src="NebulaAppLogo" class="app-logo h-24 w-auto rounded-full mb-4" />', content)

# Also update the span and container
new_content = new_content.replace('<ProgressBar class="loading-bar" :progress="Math.min(loadingProgress, 100)" />', 
                                  '<ProgressBar class="loading-bar" :progress="Math.min(loadingProgress, 100)" />')
new_content = new_content.replace('<span v-if="message">{{ message }}</span>', 
                                  '<span v-if="message" class="text-contrast font-bold text-lg mt-2">{{ message }}</span>')

with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print("Success")
