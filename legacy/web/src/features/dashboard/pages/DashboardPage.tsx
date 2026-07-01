import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { FormEvent, useState } from "react";
import { useAuth } from "../../auth/hooks/useAuth";

interface CourseSummary {
  courseId: string;
  title: string;
  description: string | null;
  lessonCount: number;
  totalCards: number;
}

interface GraphQLResponse<T> {
  data?: T;
  errors?: Array<{ message: string }>;
}

const GRAPHQL_URL = `${import.meta.env.BASE_URL}api/graphql`;

async function graphqlRequest<T>(
  query: string,
  token: string,
  variables?: Record<string, unknown>,
): Promise<T> {
  const response = await fetch(GRAPHQL_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ query, variables }),
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: GraphQL request failed`);
  }

  const result = (await response.json()) as GraphQLResponse<T>;
  if (result.errors?.length) {
    throw new Error(result.errors[0].message);
  }
  if (!result.data) {
    throw new Error("No data returned from GraphQL query");
  }

  return result.data;
}

export function DashboardPage() {
  const { user, logout } = useAuth();
  const queryClient = useQueryClient();
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  const courses = useQuery({
    queryKey: ["my-courses"],
    queryFn: async () => {
      if (!user?.token) return [];
      const data = await graphqlRequest<{
        courses: { myCourses: CourseSummary[] };
      }>(
        `query {
          courses {
            myCourses {
              courseId
              title
              description
              lessonCount
              totalCards
            }
          }
        }`,
        user.token,
      );

      return data.courses.myCourses;
    },
    enabled: !!user?.token,
  });

  const createCourse = useMutation({
    mutationFn: async () => {
      if (!user?.token) throw new Error("Not signed in");
      return graphqlRequest(
        `mutation($input: CreateCourseInput!) {
          courses {
            createCourse(input: $input) {
              courseId
              title
            }
          }
        }`,
        user.token,
        {
          input: {
            title: title.trim(),
            description: description.trim() || null,
          },
        },
      );
    },
    onSuccess: async () => {
      setTitle("");
      setDescription("");
      await queryClient.invalidateQueries({ queryKey: ["my-courses"] });
    },
  });

  const submitCourse = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!title.trim()) return;
    createCourse.mutate();
  };

  return (
    <div className="min-h-screen bg-slate-50 text-slate-950">
      <header className="border-b border-slate-200 bg-white">
        <div className="mx-auto flex max-w-5xl items-center justify-between px-4 py-4">
          <div>
            <h1 className="text-xl font-semibold">Spaced Repetition</h1>
            <p className="text-sm text-slate-600">{user?.email}</p>
          </div>
          <button
            onClick={logout}
            className="rounded-md border border-slate-300 px-3 py-2 text-sm font-medium hover:bg-slate-100"
          >
            Sign out
          </button>
        </div>
      </header>

      <main className="mx-auto grid max-w-5xl gap-6 px-4 py-6 md:grid-cols-[minmax(0,1fr)_320px]">
        <section>
          <div className="mb-4 flex items-end justify-between gap-4">
            <div>
              <h2 className="text-lg font-semibold">My Courses</h2>
              <p className="text-sm text-slate-600">
                Courses are the top-level collections for lessons and cards.
              </p>
            </div>
            {courses.data ? (
              <span className="text-sm text-slate-500">
                {courses.data.length} total
              </span>
            ) : null}
          </div>

          {courses.isLoading ? (
            <div className="rounded-md border border-slate-200 bg-white p-4 text-sm text-slate-600">
              Loading courses...
            </div>
          ) : courses.isError ? (
            <div className="rounded-md border border-red-200 bg-red-50 p-4 text-sm text-red-700">
              {(courses.error as Error).message}
            </div>
          ) : courses.data?.length ? (
            <div className="grid gap-3">
              {courses.data.map((course) => (
                <article
                  key={course.courseId}
                  className="rounded-md border border-slate-200 bg-white p-4"
                >
                  <div className="flex items-start justify-between gap-4">
                    <div>
                      <h3 className="font-medium">{course.title}</h3>
                      {course.description ? (
                        <p className="mt-1 text-sm text-slate-600">
                          {course.description}
                        </p>
                      ) : null}
                    </div>
                    <div className="shrink-0 text-right text-sm text-slate-500">
                      <div>{course.lessonCount} lessons</div>
                      <div>{course.totalCards} cards</div>
                    </div>
                  </div>
                </article>
              ))}
            </div>
          ) : (
            <div className="rounded-md border border-slate-200 bg-white p-6">
              <h3 className="font-medium">No courses yet</h3>
              <p className="mt-1 text-sm text-slate-600">
                Create a course to start organizing spaced repetition lessons.
              </p>
            </div>
          )}
        </section>

        <aside>
          <form
            onSubmit={submitCourse}
            className="rounded-md border border-slate-200 bg-white p-4"
          >
            <h2 className="text-lg font-semibold">Create Course</h2>
            <label className="mt-4 block text-sm font-medium" htmlFor="title">
              Title
            </label>
            <input
              id="title"
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              className="mt-1 w-full rounded-md border border-slate-300 px-3 py-2"
              placeholder="Japanese vocabulary"
              required
            />
            <label
              className="mt-4 block text-sm font-medium"
              htmlFor="description"
            >
              Description
            </label>
            <textarea
              id="description"
              value={description}
              onChange={(event) => setDescription(event.target.value)}
              className="mt-1 min-h-24 w-full rounded-md border border-slate-300 px-3 py-2"
              placeholder="Optional notes"
            />
            {createCourse.isError ? (
              <p className="mt-3 text-sm text-red-700">
                {(createCourse.error as Error).message}
              </p>
            ) : null}
            <button
              type="submit"
              disabled={!title.trim() || createCourse.isPending}
              className="mt-4 w-full rounded-md bg-slate-900 px-3 py-2 text-sm font-semibold text-white disabled:cursor-not-allowed disabled:opacity-60"
            >
              {createCourse.isPending ? "Creating..." : "Create course"}
            </button>
          </form>
        </aside>
      </main>
    </div>
  );
}
