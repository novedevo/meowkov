import markovify

# Get raw text as string.
with open("filtered_noumena.txt") as f:
    text = f.read()

# Build the model.
text_model = markovify.Text(text, state_size=2, well_formed=False)
text_model.compile(inplace=True)

# Print thousands of randomly-generated sentences
for i in range(6000):
    print(text_model.make_short_sentence(300, tries=500, max_overlap_ratio=0.4, max_overlap_total=10))
