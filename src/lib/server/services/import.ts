import { noteInput } from "$lib/validation/note";

export function parseBulkNotes(text: string) {
  return text
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .map((line) => {
      const [term, definition, tags = ""] = line.split("\t");
      return noteInput.parse({
        term,
        definition,
        tags: tags
          .split(",")
          .map((tag) => tag.trim())
          .filter(Boolean)
      });
    });
}
