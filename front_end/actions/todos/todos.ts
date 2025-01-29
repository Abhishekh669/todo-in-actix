"use server"

import { BACKENDURI } from "@/lib/data"
import axios from "axios"

export const  get_todos = async () =>{
    try {
        const res = await axios.get(`${BACKENDURI}/todos`);
        const data  = await res.data;
        console.log("this is form me  : ",data)
        return {
            data : JSON.stringify(data)
        }
        
    } catch (error) {
        return {error : "Failed to get data"}
    }
}

interface todo_props {
    title : string,
    description : string,
    state : string,
    tag  : string,
    position : number
}

export const create_todo = async (values : todo_props) =>{
    try {
        const res = await axios.post(`${BACKENDURI}/todo`,values)
        const data = await res.data;

        console.log("after creating: ",data)
        return {
            message : "created successfully",
            data : JSON.stringify(data)
        }
    } catch (error) {
        return {error : "Failed to create todo"}
        
    }
}


export const delete_todo = async( id : string) =>{
    try {
        const res = await axios.delete(`${BACKENDURI}/todo/${id}`);
        const data = await res.data;
        if(!data){
            throw new Error("Something went wrong")
        }
        return {
            message : "Successfully deleted message",
            data  : JSON.stringify(data)
        }
    } catch (error) {
        return {
            error : "Failed to delete the data"
        }
        
    }
}