"use client"

import { useMemo, useState } from "react"
import { Plus } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useGetTodos } from "@/utils/hooks/query-hooks/use-get-todos"
import { useCreateTodo } from "@/utils/hooks/mutate-hooks/use-create-todo"
import toast from "react-hot-toast"
import { useDeleteTodo } from "@/utils/hooks/mutate-hooks/use-delete-todo"

interface TodoType{
    _id: {
      $oid : string
    };         // The ObjectId as a string (converted from MongoDB's ObjectId)
  title: string;
  position: number;
  description: string;
  tag: string;         
  state: string;       // Same as above for state: 'PENDING', 'COMPLETED', etc.
  date: string;        // You may want to parse this date into a JavaScript Date object
  created_at: string; 
}

export default function Todo() {
    const [todo, setTodo] = useState<string>("")
    
    const {data} = useGetTodos();
    const {mutate: create_todo} = useCreateTodo();
    const {mutate : delete_todo} = useDeleteTodo();
    const todos: TodoType[] = useMemo(()=> data?.data || [],[data?.data])
    
    

    const add_todo = () =>{
        create_todo({
            title : todo,
            description : "hello world",
            state  : "PENDING",
            tag : "LOW",
            position : 1001
        },{
          onSuccess : (res) =>{
            if(res.message && res.data){
              setTodo("")
              toast.success(res.message)
            }
            else{
              throw new Error(res.error)
            }
          },
            onError : () =>{
              toast.error("Failed to create todo")
            }
        })
    }
    const handle_delete = (id : string) =>{

      delete_todo(id ,{
      onSuccess : (res) =>{
        if(res.message && res.data){
          setTodo("")
          toast.success(res.message)
        }
        else{
          throw new Error(res.error)
        }
      },
        onError : () =>{
          toast.error("Failed to delete todo")
        }
    })
    }

   

  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>My Todo List</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex space-x-2 mb-4">
            <Input
              value={todo}
              onChange={(e)=> setTodo(e.target.value)}
              type="text"
              placeholder="Add a new todo"
            />
            <Button 
                onClick={add_todo}
            >
              <Plus className="h-4 w-4 mr-2" />
              Add
            </Button>
          </div>
          <div>
            <ul className="flex flex-col gap-y-3">
              {todos && todos.map( todo => (
                <div key={todo._id?.$oid} className="border p-1 rounded-[5px]  flex justify-between items-center">
                  <span>
                    {todo.title}
                    
                    <p>{todo.description}</p>
                  </span>
                  
                  <Button
                    onClick={()=> handle_delete(todo._id.$oid)}
                  >
                    Delete
                  </Button>
                </div>
              ))}
            </ul>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}

