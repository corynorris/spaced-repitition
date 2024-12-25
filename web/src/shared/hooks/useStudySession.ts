// import { useQuery, useQueryClient } from "@tanstack/react-query";
// import { useState } from "react";

// const useStudySession = (deckId: string) => {
//   const [cards, setCards] = useState<Card[]>([]);
//   const queryClient = useQueryClient();

//   // Fetch next cards due for review
//   const { data: sessionCards } = useQuery({
//     queryKey: ["study", deckId],
//     queryFn: () => api.study.getNextCards(deckId),
//   });

//   // Submit review
//   const submitReview = useMutation({
//     mutationFn: (review: ReviewData) => api.study.submitReview(review),
//     onSuccess: () => {
//       queryClient.invalidateQueries(["study", deckId]);
//     },
//   });

//   return { cards, submitReview };
// };
