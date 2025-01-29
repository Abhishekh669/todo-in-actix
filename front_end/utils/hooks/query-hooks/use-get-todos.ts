import { get_todos } from "@/actions/todos/todos";
import { useQuery } from "@tanstack/react-query";

export  const fetch_todos = async() =>{
    const response = await  get_todos();
    return {
        data : JSON.parse(response.data as string)
    };
}

export const useGetTodos  = () =>{
    return useQuery({
        queryKey : ["get_todos"],
        queryFn : () => fetch_todos(),
    })
}