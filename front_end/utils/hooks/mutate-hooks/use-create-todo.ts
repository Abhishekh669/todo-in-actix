
import { create_todo } from "@/actions/todos/todos";
import { useMutation, useQueryClient } from "@tanstack/react-query";
export const useCreateTodo = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: create_todo,
    
    onSuccess: () => {
      
      queryClient.invalidateQueries({ queryKey: ['get_todos'] })
    },
    onError: () => { },
    onSettled: () => { },
    onMutate: () => { },
})
}